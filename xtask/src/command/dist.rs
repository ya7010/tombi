use std::{
    fs::File,
    io::{self, BufWriter},
    path::{Path, PathBuf},
};

use flate2::{write::GzEncoder, Compression};
use time::OffsetDateTime;
use xshell::Shell;
use zip::{write::FileOptions, DateTime, ZipWriter};

use crate::utils::project_root;

const DEV_VERSION: &str = "0.0.0";

pub fn run(sh: &Shell) -> Result<(), anyhow::Error> {
    let project_root = project_root();
    let target = Target::get(&project_root);
    let dist = project_root.join("dist");

    println!("Target: {:#?}", target);

    sh.remove_path(&dist)?;
    sh.create_dir(&dist)?;

    dist_server(sh, &target)?;
    dist_client(sh, &target)?;

    Ok(())
}

fn dist_server(sh: &Shell, target: &Target) -> Result<(), anyhow::Error> {
    let _e = sh.push_env("CARGO_PROFILE_RELEASE_LTO", "true");

    let target_name = &target.name;

    if target_name.contains("-linux-") {
        std::env::set_var("CC", "clang");
    }

    let manifest_path = project_root()
        .join("rust")
        .join("tombi-cli")
        .join("Cargo.toml");

    xshell::cmd!(
        sh,
        "cargo build --manifest-path {manifest_path} --bin tombi --target {target_name} --release"
    )
    .run()?;

    let dist = project_root().join("dist").join(&target.artifact_name);
    gzip(&target.server_path, &dist.with_extension("gz"))?;
    if target_name.contains("-windows-") {
        zip(
            &target.server_path,
            target.symbols_path.as_ref(),
            &dist.with_extension("zip"),
        )?;
    }

    Ok(())
}

fn gzip(src_path: &Path, dest_path: &Path) -> anyhow::Result<()> {
    let mut encoder = GzEncoder::new(File::create(dest_path)?, Compression::best());
    let mut input = std::io::BufReader::new(File::open(src_path)?);
    std::io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

fn zip(src_path: &Path, symbols_path: Option<&PathBuf>, dest_path: &Path) -> anyhow::Result<()> {
    let file = File::create(dest_path)?;
    let mut writer = ZipWriter::new(BufWriter::new(file));
    writer.start_file(
        src_path.file_name().unwrap().to_str().unwrap(),
        FileOptions::<()>::default()
            .last_modified_time(
                DateTime::try_from(OffsetDateTime::from(
                    std::fs::metadata(src_path)?.modified()?,
                ))
                .unwrap(),
            )
            .unix_permissions(0o755)
            .compression_method(zip::CompressionMethod::Deflated)
            .compression_level(Some(9)),
    )?;
    let mut input = io::BufReader::new(File::open(src_path)?);
    io::copy(&mut input, &mut writer)?;
    if let Some(symbols_path) = symbols_path {
        writer.start_file(
            symbols_path.file_name().unwrap().to_str().unwrap(),
            FileOptions::<()>::default()
                .last_modified_time(
                    DateTime::try_from(OffsetDateTime::from(
                        std::fs::metadata(src_path)?.modified()?,
                    ))
                    .unwrap(),
                )
                .compression_method(zip::CompressionMethod::Deflated)
                .compression_level(Some(9)),
        )?;
        let mut input = io::BufReader::new(File::open(symbols_path)?);
        io::copy(&mut input, &mut writer)?;
    }
    writer.finish()?;
    Ok(())
}

fn dist_client(sh: &Shell, target: &Target) -> Result<(), anyhow::Error> {
    dist_editor_vscode(sh, target)
}

fn dist_editor_vscode(sh: &Shell, target: &Target) -> Result<(), anyhow::Error> {
    let vscode_path = project_root().join("editors").join("vscode");
    let bundle_path = vscode_path.join("server");
    sh.remove_path(&bundle_path)?;
    sh.create_dir(&bundle_path)?;

    if !target.server_path.exists() {
        return Err(anyhow::anyhow!(
            "CLI binary not found at {}. Please run `cargo build --package tombi-cli --release` first.",
            target.server_path.display()
        ));
    }

    sh.copy_file(&target.server_path, &bundle_path.join("tombi"))?;
    if let Some(symbols_path) = &target.symbols_path {
        sh.copy_file(symbols_path, &bundle_path)?;
    }

    let _d: xshell::PushDir<'_> = sh.push_dir(vscode_path);
    let mut patch = Patch::new(sh, "package.json")?;
    patch.replace(
        &format!(r#""version": "{}""#, DEV_VERSION),
        &format!(r#""version": "{}""#, target.version),
    );
    patch.commit(sh)?;

    Ok(())
}

#[derive(Debug)]
struct Target {
    name: String,
    version: String,
    server_path: PathBuf,
    symbols_path: Option<PathBuf>,
    artifact_name: String,
}

impl Target {
    fn get(project_root: &Path) -> Self {
        let name = match std::env::var("TOMBI_TARGET") {
            Ok(target) => target,
            _ => {
                if cfg!(target_os = "linux") {
                    "x86_64-unknown-linux-gnu".to_owned()
                } else if cfg!(target_os = "windows") {
                    "x86_64-pc-windows-msvc".to_owned()
                } else if cfg!(target_os = "macos") {
                    "aarch64-apple-darwin".to_owned()
                } else {
                    panic!("Unsupported OS, maybe try setting TOMBI_TARGET")
                }
            }
        };
        let version = match std::env::var("GITHUB_REF") {
            Ok(github_ref) if github_ref.starts_with("refs/tags/v") => {
                github_ref.trim_start_matches("refs/tags/v").to_owned()
            }
            _ => DEV_VERSION.to_owned(),
        };
        let out_path = project_root.join("target").join(&name).join("release");
        let (exe_suffix, symbols_path) = if name.contains("-windows-") {
            (".exe".into(), Some(out_path.join("tombi.pdb")))
        } else {
            (String::new(), None)
        };
        let server_path = out_path.join(format!("tombi{exe_suffix}"));
        let artifact_name = format!("tombi-{name}{exe_suffix}");

        Self {
            name,
            version,
            server_path,
            symbols_path,
            artifact_name,
        }
    }
}

struct Patch {
    path: PathBuf,
    contents: String,
}

impl Patch {
    fn new(sh: &Shell, path: impl Into<PathBuf>) -> anyhow::Result<Patch> {
        let path = path.into();
        let contents = sh.read_file(&path)?;
        Ok(Patch { path, contents })
    }

    fn replace(&mut self, from: &str, to: &str) -> &mut Patch {
        assert!(self.contents.contains(from));
        self.contents = self.contents.replace(from, to);
        self
    }

    fn commit(&self, sh: &Shell) -> anyhow::Result<()> {
        sh.write_file(&self.path, &self.contents)?;
        Ok(())
    }
}
