use diagnostics::{printer::Pretty, Diagnostic, OkOrErrPrint, Print};

use crate::app::arg;
use std::io::Read;

/// Format TOML files.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    files: Vec<String>,

    /// Check if the input is formatted.
    #[arg(long, default_value_t = false)]
    check: bool,
}

pub fn run(args: Args) -> Result<(), crate::Error> {
    tracing::debug_span!("run").in_scope(|| {
        tracing::debug!("{args:?}");

        let (success_num, error_num) = inner_run(args, Pretty);
        if error_num > 0 {
            std::process::exit(1);
        }
        if success_num > 0 {
            eprintln!("{} files formatted", success_num);
        }

        Ok(())
    })
}

fn inner_run<P>(args: Args, printer: P) -> (usize, usize)
where
    Diagnostic: Print<P>,
    crate::Error: Print<P>,
    P: Copy,
{
    let input = arg::FileInput::from(args.files.as_ref());

    let total_num = input.len();
    let mut success_num = 0;
    let mut error_num = 0;

    match input {
        arg::FileInput::Stdin => {
            tracing::debug!("stdin input formatting...");
            if format_file(std::io::stdin(), printer, &args) {
                success_num += 1;
            } else {
                error_num += 1;
            }
        }
        arg::FileInput::Files(files) => {
            for file in files {
                match file {
                    Ok(path) => {
                        tracing::debug!("{:?} formatting...", path);
                        match std::fs::File::open(&path) {
                            Ok(file) => {
                                if format_file(file, printer, &args) {
                                    success_num += 1;
                                    continue;
                                }
                            }
                            Err(err) => {
                                if err.kind() == std::io::ErrorKind::NotFound {
                                    crate::Error::FileNotFound(path).print(printer);
                                } else {
                                    crate::Error::Io(err).print(printer);
                                }
                            }
                        }
                    }
                    Err(err) => err.print(printer),
                }
                error_num += 1;
            }
        }
    };

    assert_eq!(success_num + error_num, total_num);

    (success_num, error_num)
}

fn format_file<R: Read, P>(mut reader: R, printer: P, args: &Args) -> bool
where
    Diagnostic: Print<P>,
    crate::Error: Print<P>,
    P: Copy,
{
    let mut source = String::new();
    if reader.read_to_string(&mut source).is_ok() {
        if let Some(formatted) =
            formatter::format_with_option(&source, &Default::default()).ok_or_err_print(printer)
        {
            if args.check && source != formatted {
                crate::error::NotFormattedError::from_input()
                    .into_error()
                    .print(printer);
            } else {
                return true;
            }
        }
    }
    false
}
