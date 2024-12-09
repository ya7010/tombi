use crate::{hover::HoverContent, toml};
use ast::{algo::ancestors_at_position, AstNode};
use config::TomlVersion;
use itertools::Itertools;
use parser::SyntaxKind::*;
use tower_lsp::lsp_types::{Hover, HoverParams, Position, TextDocumentPositionParams};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_hover(
    HoverParams {
        text_document_position_params:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    }: HoverParams,
) -> Result<Option<Hover>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_hover");

    let source = toml::try_load(&text_document.uri)?;

    let Some(root) =
        ast::Root::cast(parser::parse(&source, TomlVersion::default()).into_syntax_node())
    else {
        return Ok(None);
    };

    if let Some(hover_content) = get_hover_content(root, position) {
        Ok(Some(hover_content.into_hover()))
    } else {
        Ok(None)
    }
}

fn get_hover_content(root: ast::Root, position: Position) -> Option<HoverContent> {
    // NOTE: Eventually, only KeyValue, Table, ArrayOfTables may be shown in the hover.
    //       For now, all nodes are displayed for debugging purposes.

    let mut is_key_value = false;
    let mut is_keys = false;
    for node in ancestors_at_position(root.syntax(), position.into()) {
        if let Some(key) = ast::Key::cast(node.to_owned()) {
            let keys = key
                .syntax()
                .ancestors()
                .filter_map(|node| match node.kind() {
                    KEYS => {
                        is_key_value = false;
                        is_keys = true;
                        ast::Keys::cast(node).map(|keys| {
                            keys.keys()
                                .filter(|k| {
                                    k.syntax().text_range().start()
                                        <= key.syntax().text_range().start()
                                })
                                .collect_vec()
                        })
                    }
                    KEY_VALUE => {
                        is_key_value = true;
                        if !is_keys {
                            ast::KeyValue::cast(node)
                                .map(|kv| kv.keys().unwrap().keys().collect_vec())
                        } else {
                            None
                        }
                    }
                    TABLE => is_key_value
                        .then(|| {
                            ast::Table::cast(node)
                                .map(|table| table.header().map(|keys| keys.keys().collect_vec()))
                                .flatten()
                        })
                        .flatten(),
                    ARRAY_OF_TABLES => is_key_value
                        .then(|| {
                            ast::ArrayOfTables::cast(node)
                                .map(|array| array.header().map(|keys| keys.keys().collect_vec()))
                                .flatten()
                        })
                        .flatten(),
                    _ => {
                        is_keys = false;
                        None
                    }
                })
                .collect_vec()
                .into_iter()
                .rev()
                .flatten()
                .map(|key| key.syntax().text().to_string())
                .join(".");

            return Some(HoverContent {
                keys,
                range: key.syntax().text_range(),
                ..Default::default()
            });
        }
    }
    None
}
