mod backend;
mod completion;
mod document;
mod handler;
mod hover;
mod semantic_tokens;

use backend::Backend;

/// Run TOML Language Server
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Args {}

pub async fn serve(_args: impl Into<Args>) {
    tracing::info!(
        "Tombi LSP Server Version \"{}\" will start.",
        env!("CARGO_PKG_VERSION")
    );

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = tower_lsp::LspService::build(crate::backend::Backend::new)
        .custom_method("tombi/getTomlVersion", Backend::get_toml_version)
        .custom_method("tombi/updateSchema", Backend::update_schema)
        .custom_method("tombi/updateConfig", Backend::update_config)
        .finish();

    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;

    tracing::info!("Tombi LSP Server did shut down.");
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    fn project_root() -> PathBuf {
        let dir = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
        PathBuf::from(dir)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned()
    }

    pub fn tombi_schema_path() -> PathBuf {
        project_root().join("tombi.schema.json")
    }

    pub fn cargo_schema_path() -> PathBuf {
        project_root().join("schemas").join("cargo.schema.json")
    }

    pub fn pyproject_schema_path() -> PathBuf {
        project_root().join("schemas").join("pyproject.schema.json")
    }
}
