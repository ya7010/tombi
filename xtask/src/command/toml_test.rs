use xshell::Shell;

use crate::utils::project_root;

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let project_root = project_root();

    sh.change_dir(&project_root);

    xshell::cmd!(sh, "cargo build --bin decode").run()?;

    xshell::cmd!(
        sh,
        "toml-test -color=never {project_root}/target/debug/decode > {project_root}/toml-test/result/decode.txt"
    ).run().unwrap();

    Ok(())
}
