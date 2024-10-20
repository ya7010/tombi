use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use diagnostics::Diagnostic;
use diagnostics::{Pretty, Print};
use text_size::TextRange;
use tracing_subscriber::prelude::*;

#[derive(clap::Parser)]
pub struct Args {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

pub fn project_root() -> PathBuf {
    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

pub fn source_file() -> PathBuf {
    project_root().join("Cargo.toml")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse_from(std::env::args_os());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(
            args.verbose.log_level_filter().to_string(),
        ))
        .with(tracing_subscriber::fmt::layer().pretty().without_time())
        .init();

    let source_file = source_file();
    println!("source_file: {:?}", source_file);
    let source = std::fs::read_to_string(&source_file)?;

    let warning = Diagnostic::new_warnig(
        "Some warning occured.".to_owned(),
        &source,
        TextRange::new(0.into(), 10.into()),
    );
    let error = Diagnostic::new_error(
        "Some error occured.".to_owned(),
        &source,
        TextRange::new(12.into(), 20.into()),
    );

    Pretty.print(&warning);
    Pretty.print(&warning.with_source_file(&source_file));

    Pretty.print(&error);
    Pretty.print(&error.with_source_file(&source_file));
    Ok(())
}
