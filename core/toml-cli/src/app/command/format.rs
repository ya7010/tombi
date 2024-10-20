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
                    let formatted = formatter::format_with_option(&source, &Default::default())?;

                    if args.check && source != formatted {
                        eprintln!("Error: input is not formatted");
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
                                    if let Ok(_) = file.read_to_string(&mut source) {
                                        let formatted = formatter::format_with_option(
                                            &source,
                                            &Default::default(),
                                        )?;
                                        if args.check && source != formatted {
                                            eprintln!("Error: input is not formatted");
                                        }
                                    }
                                }
                                Err(err) => eprintln!("Error: {:?}", err),
                            }
                        }
                        Err(err) => eprintln!("Error: {:?}", err),
                    }
                }
            }
        }

        Ok(())
    })
}
