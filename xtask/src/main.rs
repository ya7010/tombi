mod app;
mod codegen;
mod command;
mod error;
mod utils;

pub use error::Error;

fn main() {
    if let Err(err) = app::run(std::env::args_os()) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
