use document_tree::IntoDocumentTreeAndErrors;
use itertools::Either;
use reqwest::Client;
use schema_store::SchemaContext;
use tower_lsp::lsp_types::{
    request::{GotoTypeDefinitionParams, GotoTypeDefinitionResponse},
    CreateFile, CreateFileOptions, DocumentChangeOperation, DocumentChanges, Location, OneOf,
    OptionalVersionedTextDocumentIdentifier, Position, Range, ResourceOp, TextDocumentEdit,
    TextEdit, Url, WorkspaceEdit,
};

use crate::{
    backend::Backend,
    handler::hover::get_hover_range,
    hover::{get_hover_content, HoverContent},
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_goto_type_definition(
    backend: &Backend,
    params: GotoTypeDefinitionParams,
) -> Result<Option<GotoTypeDefinitionResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_goto_type_definition");
    tracing::trace!(?params);

    let GotoTypeDefinitionParams {
        text_document_position_params:
            tower_lsp::lsp_types::TextDocumentPositionParams {
                text_document,
                position,
                ..
            },
        ..
    } = params;

    let config = backend.config().await;

    if !config
        .server
        .and_then(|server| server.goto_type_definition)
        .and_then(|goto_type_definition| goto_type_definition.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`server.goto_type_definition.enabled` is false");
        return Ok(None);
    }

    let position = position.into();
    let Some(root) = backend.get_incomplete_ast(&text_document.uri).await else {
        return Ok(None);
    };

    let source_schema = backend
        .schema_store
        .try_get_source_schema_from_ast(&root, Some(Either::Left(&text_document.uri)))
        .await
        .ok()
        .flatten();

    let (toml_version, _) = backend.source_toml_version(source_schema.as_ref()).await;

    let Some((keys, range)) = get_hover_range(&root, position, toml_version).await else {
        return Ok(None);
    };

    if keys.is_empty() && range.is_none() {
        return Ok(None);
    }

    let document_tree = root.into_document_tree_and_errors(toml_version).tree;

    Ok(
        match get_hover_content(
            &document_tree,
            position,
            &keys,
            &SchemaContext {
                toml_version,
                root_schema: source_schema.as_ref().and_then(|s| s.root_schema.as_ref()),
                sub_schema_url_map: source_schema.as_ref().map(|s| &s.sub_schema_url_map),
                store: &backend.schema_store,
            },
        )
        .await
        {
            Some(HoverContent {
                schema_url: Some(schema_url),
                ..
            }) => {
                let url: Url = schema_url.into();
                if matches!(url.scheme(), "http" | "https") {
                    // Fetch the remote content
                    let client = Client::new();
                    let content = match client.get(url.to_string()).send().await {
                        Ok(response) => match response.text().await {
                            Ok(content) => content,
                            Err(e) => {
                                tracing::error!("Error fetching content: {}", e);
                                return Err(tower_lsp::jsonrpc::Error::new(
                                    tower_lsp::jsonrpc::ErrorCode::InternalError,
                                ));
                            }
                        },
                        Err(e) => {
                            tracing::error!("Error fetching content: {}", e);
                            return Err(tower_lsp::jsonrpc::Error::new(
                                tower_lsp::jsonrpc::ErrorCode::InternalError,
                            ));
                        }
                    };

                    // Create a new file with the content
                    let virtual_file_uri =
                        Url::parse(&format!("untitled://{}", url.path())).unwrap();

                    // First, create the file
                    let create_file = CreateFile {
                        uri: virtual_file_uri.clone(),
                        options: Some(CreateFileOptions {
                            overwrite: Some(true),
                            ignore_if_exists: Some(false),
                        }),
                        annotation_id: None,
                    };

                    // Create a workspace edit with both changes
                    let edit = WorkspaceEdit {
                        changes: None,
                        document_changes: Some(DocumentChanges::Operations(vec![
                            DocumentChangeOperation::Op(ResourceOp::Create(create_file)),
                        ])),
                        change_annotations: None,
                    };

                    // Apply the workspace edit
                    let _ = backend
                        .client
                        .send_request::<tower_lsp::lsp_types::request::ApplyWorkspaceEdit>(
                            tower_lsp::lsp_types::ApplyWorkspaceEditParams {
                                label: Some("Create remote file".to_string()),
                                edit,
                            },
                        )
                        .await;

                    // Then, create the text document edit
                    let text_document_edit = TextDocumentEdit {
                        text_document: OptionalVersionedTextDocumentIdentifier {
                            uri: virtual_file_uri.clone(),
                            version: Some(0),
                        },
                        edits: vec![OneOf::Left(TextEdit {
                            range: Range {
                                start: Position::new(0, 0),
                                end: Position::new(0, 0),
                            },
                            new_text: content,
                        })],
                    };

                    // Create a workspace edit with both changes
                    let edit = WorkspaceEdit {
                        changes: None,
                        document_changes: Some(DocumentChanges::Edits(vec![text_document_edit])),
                        change_annotations: None,
                    };

                    // Apply the workspace edit
                    let _ = backend
                        .client
                        .send_request::<tower_lsp::lsp_types::request::ApplyWorkspaceEdit>(
                            tower_lsp::lsp_types::ApplyWorkspaceEditParams {
                                label: Some("Create remote file".to_string()),
                                edit,
                            },
                        )
                        .await;

                    Some(GotoTypeDefinitionResponse::Scalar(Location {
                        uri: virtual_file_uri,
                        range: text::Range::default().into(),
                    }))
                } else {
                    Some(GotoTypeDefinitionResponse::Scalar(Location {
                        uri: url,
                        range: text::Range::default().into(),
                    }))
                }
            }
            _ => None,
        },
    )
}
