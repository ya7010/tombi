use std::io::Read;

/// Format TOML files.
#[derive(clap::Args)]
pub struct Args {
    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    files: Vec<String>,

    /// Set the line-length
    #[arg(long, default_value = None)]
    pub max_line_length: Option<u8>,
}

pub fn run(args: Args) -> Result<(), crate::Error> {
    match crate::arg::FileInput::from(args.files) {
        crate::arg::FileInput::Stdin => {
            let mut buffer = String::new();
            if let Ok(_) = std::io::stdin().read_to_string(&mut buffer) {
                formatter::format(&buffer, &Default::default())?;
            }
        }
        crate::arg::FileInput::Files(files) => {
            for file in files {
                match file {
                    Ok(path) => {
                        let mut buffer = String::new();
                        match std::fs::File::open(&path) {
                            Ok(mut file) => {
                                if let Ok(_) = file.read_to_string(&mut buffer) {
                                    formatter::format(&buffer, &Default::default())?;
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
}
