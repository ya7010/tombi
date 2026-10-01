use tombi_text::IntoLsp;

use crate::{Backend, remote_file::open_remote_file};

pub async fn into_lsp_locations(
    backend: &Backend,
    locations: Vec<tombi_extension::Location>,
) -> Result<Vec<tower_lsp::lsp_types::Location>, tower_lsp::jsonrpc::Error> {
    if locations.is_empty() {
        return Ok(Vec::new());
    }

    let mut uri_set = tombi_hashmap::HashMap::new();
    for location in &locations {
        if let Ok(Some(remote_uri)) = open_remote_file(backend, &location.uri).await {
            uri_set.insert(location.uri.clone(), remote_uri);
        }
    }

    let encoding = backend.capabilities.read().await.encoding_kind;
    let document_sources = backend.document_sources.try_read().ok();

    let mut lsp_locations = Vec::with_capacity(locations.len());
    for mut location in locations {
        if let Some(remote_uri) = uri_set.get(&location.uri) {
            location.uri = remote_uri.clone();
        }
        let range = match location.span {
            Some(tombi_extension::LocatedSpan { span, line_index }) => {
                // An open document may have unsaved changes, so its line index is preferred.
                let line_index = document_sources
                    .as_ref()
                    .and_then(|ds| ds.get(&location.uri))
                    .map_or(line_index, |document_source| {
                        document_source.line_index_arc()
                    });
                span.into_lsp(&line_index, encoding)
            }
            // A file that is not parsed is opened at its start.
            None => tower_lsp::lsp_types::Range::default(),
        };
        lsp_locations.push(tower_lsp::lsp_types::Location {
            uri: location.uri.into(),
            range,
        });
    }

    Ok(lsp_locations)
}
