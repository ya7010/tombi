use std::path::{Path, PathBuf};

use xshell::Shell;

use crate::utils::project_root;

pub fn run(sh: &Shell) -> Result<(), anyhow::Error> {
    let project_root = project_root();
    let target = Target::get(&project_root);
    let dist = project_root.join("dist");

    println!("Target: {:#?}", target);

    sh.remove_path(&dist)?;
    sh.create_dir(&dist)?;

    dist_editor_vscode(sh, &target)?;

    Ok(())
}

fn dist_editor_vscode(sh: &Shell, target: &Target) -> Result<(), anyhow::Error> {
    let bundle_path = Path::new("editors").join("vscode").join("server");
    sh.remove_path(&bundle_path)?;
    sh.create_dir(&bundle_path)?;

    sh.copy_file(&target.cli_path, &bundle_path.join("tombi"))?;
    if let Some(symbols_path) = &target.symbols_path {
        sh.copy_file(symbols_path, &bundle_path)?;
    }

    let _d = sh.push_dir("./editors/vscode");
    let mut patch = Patch::new(sh, "./package.json")?;
    patch.replace(
        &format!(r#""version": "0.0.0-dev""#),
        &format!(r#""version": "{}""#, target.version),
    );
    patch.commit(sh)?;

    Ok(())
}

#[derive(Debug)]
struct Target {
    name: String,
    version: String,
    cli_path: PathBuf,
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
                    "x86_64-apple-darwin".to_owned()
                } else {
                    panic!("Unsupported OS, maybe try setting TOMBI_TARGET")
                }
            }
        };
        let version = match std::env::var("GITHUB_REF") {
            Ok(github_ref) if github_ref.starts_with("refs/tags/v") => {
                github_ref.trim_start_matches("refs/tags/v").to_owned()
            }
            _ => "0.0.0".to_owned(),
        };
        let out_path = project_root.join("target").join(&name).join("release");
        let (exe_suffix, symbols_path) = if name.contains("-windows-") {
            (".exe".into(), Some(out_path.join("tombi.pdb")))
        } else {
            (String::new(), None)
        };
        let cli_path = out_path.join(format!("tombi{exe_suffix}"));
        let artifact_name = format!("tombi-{name}{exe_suffix}");

        Self {
            name,
            version,
            cli_path,
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
