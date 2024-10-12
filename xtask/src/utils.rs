pub use crate::glue::pushenv;
use crate::run;
use std::{env, path::PathBuf};

/// Returns the path to the root directory of `rust-analyzer` project.
pub fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir).parent().unwrap().to_owned()
}

pub fn reformat(text: impl std::fmt::Display) -> Result<String, anyhow::Error> {
    reformat_without_preamble(text).map(prepend_generated_preamble)
}

pub const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/src/codegen`";
pub fn prepend_generated_preamble(content: impl std::fmt::Display) -> String {
    format!("//! {PREAMBLE}\n\n{content}")
}

pub fn reformat_without_preamble(text: impl std::fmt::Display) -> Result<String, anyhow::Error> {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    let output = run!(
        "rustfmt --config newline_style=Unix";
        <text.to_string().as_bytes()
    )?;

    Ok(format!("{output}\n"))
}

pub fn ensure_rustfmt() -> Result<(), anyhow::Error> {
    let out = run!("rustfmt --version")?;
    if !out.contains("stable") {
        anyhow::bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
    Ok(())
}
