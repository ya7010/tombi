fn main() -> Result<(), anyhow::Error> {
    if let Err(err) = tombi_lsp::app::run(std::env::args_os()) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
