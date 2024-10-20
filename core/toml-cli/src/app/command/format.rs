use diagnostics::{printer::Pretty, OkOrPrint, Print};

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
        match arg::FileInput::from(args.files.as_ref()) {
            arg::FileInput::Stdin => {
                let mut source = String::new();
                if let Ok(_) = std::io::stdin().read_to_string(&mut source) {
                    if let Some(formatted) =
                        formatter::format_with_option(&source, &Default::default())
                            .ok_or_print(Pretty)
                    {
                        if args.check && source != formatted {
                            Err(crate::error::NotFormattedError::from_input())?;
                        }
                    }
                }
            }
            arg::FileInput::Files(files) => {
                let mut errors: Vec<crate::Error> = Vec::new();
                for file in files {
                    match file {
                        Ok(path) => {
                            let mut source = String::new();
                            match std::fs::File::open(&path) {
                                Ok(mut file) => {
                                    if file.read_to_string(&mut source).is_ok() {
                                        if let Some(formatted) = formatter::format_with_option(
                                            &source,
                                            &Default::default(),
                                        )
                                        .ok_or_print(Pretty)
                                        {
                                            if args.check && source != formatted {
                                                errors.push(Into::<crate::Error>::into(
                                                    crate::error::NotFormattedError::from_input(),
                                                ));
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    if err.kind() == std::io::ErrorKind::NotFound {
                                        errors.push(crate::Error::FileNotFound(path));
                                    } else {
                                        errors.push(Into::<crate::Error>::into(err));
                                    }
                                }
                            }
                        }
                        Err(err) => errors.push(err),
                    }
                }

                if !errors.is_empty() {
                    errors.iter().for_each(|err| err.print(Pretty));
                }
            }
        }

        Ok(())
    })
}
