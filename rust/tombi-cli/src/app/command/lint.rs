use crate::app::arg;
use config::{LintOptions, TomlVersion};
use diagnostic::{printer::Pretty, Diagnostic, Print};
use tokio::io::AsyncReadExt;

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

    /// Do not use the schema store's JSON file catalog.
    #[arg(long, action = clap::ArgAction::Set, default_value = "true")]
    use_schema_catalog: bool,
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
    let config = config::load();
    let toml_version = args
        .toml_version
        .unwrap_or(config.toml_version.unwrap_or_default());
    let options = config.lint.unwrap_or_default();
    let schema_store = schema_store::SchemaStore::default();

    let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    else {
        tracing::error!("Failed to create tokio runtime");
        std::process::exit(1);
    };

    runtime.block_on(async {
        if args.use_schema_catalog {
            let catalog_url = schema_store::DEFAULT_CATALOG_URL.parse().unwrap();
            schema_store.load_catalog(&catalog_url).await;
        }

        let input = arg::FileInput::from(args.files.as_ref());
        let total_num = input.len();
        let mut success_num = 0;
        let mut error_num = 0;

        match input {
            arg::FileInput::Stdin => {
                tracing::debug!("linting... stdin input");
                if lint_file(
                    tokio::io::stdin(),
                    printer,
                    None,
                    toml_version,
                    &options,
                    &schema_store,
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
                        Ok(source_path) => {
                            tracing::debug!("linting... {:?}", source_path);
                            match tokio::fs::File::open(&source_path).await {
                                Ok(file) => {
                                    let options = options.clone();
                                    let schema_store = schema_store.clone();

                                    tasks.spawn(async move {
                                        lint_file(
                                            file,
                                            printer,
                                            Some(source_path.as_ref()),
                                            toml_version,
                                            &options,
                                            &schema_store,
                                        )
                                        .await
                                    });
                                }
                                Err(err) => {
                                    if err.kind() == std::io::ErrorKind::NotFound {
                                        crate::Error::FileNotFound(source_path).print(printer);
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
                        Ok(success) => {
                            if success {
                                success_num += 1;
                            } else {
                                error_num += 1;
                            }
                        }
                        Err(e) => {
                            tracing::error!("task failed {}", e);
                            error_num += 1;
                        }
                    }
                }
            }
        }

        assert_eq!(success_num + error_num, total_num);

        (success_num, error_num)
    })
}

async fn lint_file<R, P>(
    mut reader: R,
    printer: P,
    source_path: Option<&std::path::Path>,
    toml_version: TomlVersion,
    options: &LintOptions,
    schema_store: &schema_store::SchemaStore,
) -> bool
where
    Diagnostic: Print<P>,
    crate::Error: Print<P>,
    P: Copy + Send,
    R: AsyncReadExt + Unpin + Send,
{
    let mut source = String::new();
    if reader.read_to_string(&mut source).await.is_ok() {
        match linter::Linter::new(toml_version, options, schema_store)
            .lint(&source)
            .await
        {
            Ok(()) => {
                return true;
            }
            Err(diagnostics) => if let Some(source_path) = source_path {
                diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.with_source_file(source_path))
                    .collect()
            } else {
                diagnostics
            }
            .print(printer),
        }
    }
    false
}
