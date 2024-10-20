use diagnostics::{printer::Pretty, Diagnostic, OkOrPrint, Print};

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

    /// Set the line-length
    #[arg(long, default_value = None)]
    pub max_line_length: Option<u8>,
}

pub fn run(args: Args) -> Result<(), crate::Error> {
    tracing::debug_span!("run").in_scope(|| {
        tracing::debug!("{args:?}");

        let success_num = inner_run(args, Pretty);
        if success_num > 0 {
            eprintln!("{} files formatted", success_num);
        }

        Ok(())
    })
}

fn inner_run<P>(args: Args, printer: P) -> usize
where
    Diagnostic: Print<P>,
    crate::Error: Print<P>,
    P: Copy,
{
    let mut formatted_num = 0;

    match arg::FileInput::from(args.files.as_ref()) {
        arg::FileInput::Stdin => {
            let mut source = String::new();
            if std::io::stdin().read_to_string(&mut source).is_ok() {
                if let Some(formatted) =
                    formatter::format_with_option(&source, &Default::default()).ok_or_print(printer)
                {
                    if args.check && source != formatted {
                        crate::error::NotFormattedError::from_input()
                            .to_error()
                            .print(printer);
                    } else {
                        formatted_num += 1;
                    }
                }
            }
        }
        arg::FileInput::Files(files) => {
            for file in files {
                match file {
                    Ok(path) => {
                        let mut source = String::new();
                        match std::fs::File::open(&path) {
                            Ok(mut file) => {
                                if file.read_to_string(&mut source).is_ok() {
                                    if let Some(formatted) =
                                        formatter::format_with_option(&source, &Default::default())
                                            .ok_or_print(printer)
                                    {
                                        if args.check && source != formatted {
                                            crate::error::NotFormattedError::from_input()
                                                .to_error()
                                                .print(printer);
                                        } else {
                                            formatted_num += 1;
                                        }
                                    }
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
            }
        }
    };

    formatted_num
}
