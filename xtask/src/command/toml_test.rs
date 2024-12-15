use std::io::Write;

use xshell::Shell;

use crate::utils::project_root;

use config::TomlVersion;

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let project_root = project_root();

    sh.change_dir(&project_root);

    xshell::cmd!(sh, "cargo build --bin decode").run()?;

    let mut has_failed = false;
    for &toml_version in &[TomlVersion::V1_0_0, TomlVersion::V1_1_0_Preview] {
        if let Ok(false) = decode_test(sh, &project_root, toml_version) {
            has_failed = true;
        }
    }

    if has_failed {
        std::process::exit(1);
    }

    Ok(())
}

fn decode_test(
    sh: &Shell,
    project_root: &std::path::Path,
    toml_version: TomlVersion,
) -> anyhow::Result<bool> {
    let toml_test_version = toml_test_version(toml_version);
    let toml_version_str = serde_json::to_string(&toml_version).unwrap();
    let toml_version_str = toml_version_str.trim_matches('"');

    match xshell::cmd!(
        sh,
        "toml-test -color=never -toml={toml_test_version} -- {project_root}/target/debug/decode --toml-version {toml_version_str}"
    )
    .ignore_status()
    .output()
    {
        Ok(output) => {
            let output_path =
                project_root.join(format!("toml-test/result/decode-{toml_version_str}.txt"));
            let mut file = std::fs::File::create(output_path)?;
            file.write_all(&output.stdout)?;
            Ok(output.status.success())
        }
        Err(_) => unreachable!("ignore_status() should prevent this"),
    }
}

const fn toml_test_version(toml_version: TomlVersion) -> &'static str {
    match toml_version {
        TomlVersion::V1_0_0 => "1.0.0",
        TomlVersion::V1_1_0_Preview => "1.1.0",
    }
}
