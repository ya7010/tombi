use std::io::Write;

use xshell::Shell;

use crate::utils::project_root;

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let project_root = project_root();

    sh.change_dir(&project_root);

    xshell::cmd!(sh, "cargo build --bin decode").run()?;

    match xshell::cmd!(
        sh,
        "toml-test -color=never {project_root}/target/debug/decode"
    )
    .ignore_status()
    .output()
    {
        Ok(output) => {
            let output_path = project_root.join("toml-test/result/decode.txt");
            let mut file = std::fs::File::create(output_path)?;
            file.write_all(&output.stdout)?;
            if !output.status.success() {
                std::process::exit(output.status.code().unwrap_or(1));
            }
        }
        Err(_) => unreachable!("ignore_status() should prevent this"),
    };

    Ok(())
}
