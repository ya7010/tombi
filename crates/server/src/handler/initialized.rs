use tower_lsp::lsp_types::{InitializedParams, MessageType};

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_initialized(backend: &Backend, InitializedParams { .. }: InitializedParams) {
    backend
        .schema_store
        .load_config_schema(
            None,
            match &backend.config().await.schemas {
                Some(schemas) => schemas,
                None => &[],
            },
        )
        .await;

    load_catalog(backend).await;
}

async fn load_catalog(backend: &Backend) {
    let config = backend.config().await;
    let Some(catalog) = config
        .schema
        .as_ref()
        .and_then(|options| options.catalog.as_ref())
    else {
        return;
    };

    let Some(catalog_paths) = catalog.paths() else {
        return;
    };

    for catalog_path in catalog_paths {
        if let Ok(catalog_url) = (&catalog_path).try_into() {
            if let Err(err) = backend
                .schema_store
                .load_catalog_from_url(&catalog_url)
                .await
            {
                let Ok(_) = backend
                    .client
                    .show_message_request(MessageType::ERROR, err.to_string(), None)
                    .await
                else {
                    continue;
                };
            }
        } else {
            let Ok(_) = backend
                .client
                .show_message_request(
                    MessageType::ERROR,
                    format!("invalid catalog url: {:?}", &catalog_path),
                    None,
                )
                .await
            else {
                continue;
            };
        }
    }
}
