use crate::app::arg;
use config::{LintOptions, TomlVersion};
use diagnostic::{printer::Pretty, Diagnostic, Print};
use std::io::Read;

/// Lint TOML files.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input is used.
    files: Vec<String>,

    /// TOML version.
    #[arg(long, value_enum, default_value = None)]
    toml_version: Option<TomlVersion>,
}

#[tracing::instrument(level = "debug", skip_all)]
pub fn run(args: Args) -> Result<(), crate::Error> {
    let (success_num, error_num) = inner_run(args, Pretty);

    match success_num {
        0 => eprintln!("No files linted"),
        1 => eprintln!("1 file linted"),
        _ => eprintln!("{} files linted", success_num),
    }

    if error_num > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn inner_run<P>(args: Args, printer: P) -> (usize, usize)
where
    Diagnostic: Print<P>,
    crate::Error: Print<P>,
    P: Copy + Clone + Send + 'static,
{
    let input = arg::FileInput::from(args.files.as_ref());
    let total_num = input.len();
    let mut success_num = 0;
    let mut error_num = 0;

    let config = config::load();
    let toml_version = args
        .toml_version
        .unwrap_or(config.toml_version.unwrap_or_default());
    let options = config.lint.unwrap_or_default();
    let mut schema_store = schema_store::SchemaStore::default();

    let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    else {
        tracing::error!("Failed to create tokio runtime");
        std::process::exit(1);
    };

    runtime.block_on(async {
        match input {
            arg::FileInput::Stdin => {
                tracing::debug!("stdin input linting...");
                if lint_file(
                    std::io::stdin(),
                    printer,
                    toml_version,
                    &options,
                    &mut schema_store,
                )
                .await
                {
                    success_num += 1;
                } else {
                    error_num += 1;
                }
            }
            arg::FileInput::Files(files) => {
                let mut tasks = tokio::task::JoinSet::new();

                for file in files {
                    match file {
                        Ok(path) => {
                            tracing::debug!("{:?} linting...", path);
                            match std::fs::File::open(&path) {
                                Ok(file) => {
                                    let options = options.clone();
                                    tasks.spawn(async move {
                                        let mut schema_store = schema_store::SchemaStore::default();
                                        (
                                            path,
                                            lint_file(
                                                file,
                                                printer,
                                                toml_version,
                                                &options,
                                                &mut schema_store,
                                            )
                                            .await,
                                        )
                                    });
                                }
                                Err(err) => {
                                    if err.kind() == std::io::ErrorKind::NotFound {
                                        crate::Error::FileNotFound(path).print(printer);
                                    } else {
                                        crate::Error::Io(err).print(printer);
                                    }
                                    error_num += 1;
                                }
                            }
                        }
                        Err(err) => {
                            err.print(printer);
                            error_num += 1;
                        }
                    }
                }

                while let Some(result) = tasks.join_next().await {
                    match result {
                        Ok((path, success)) => {
                            if success {
                                success_num += 1;
                            } else {
                                tracing::debug!("{:?} lint failed", path);
                                error_num += 1;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Task failed: {}", e);
                            error_num += 1;
                        }
                    }
                }
            }
        }
    });

    assert_eq!(success_num + error_num, total_num);
    (success_num, error_num)
}

async fn lint_file<R: Read, P>(
    mut reader: R,
    printer: P,
    toml_version: TomlVersion,
    options: &LintOptions,
    schema_store: &mut schema_store::SchemaStore,
) -> bool
where
    Diagnostic: Print<P>,
    crate::Error: Print<P>,
    P: Copy + Send,
    R: Send,
{
    let mut source = String::new();
    if reader.read_to_string(&mut source).is_ok() {
        match linter::Linter::new(toml_version, &options, schema_store)
            .lint(&source)
            .await
        {
            Ok(()) => {
                return true;
            }
            Err(diagnostics) => diagnostics.print(printer),
        }
    }
    false
}
