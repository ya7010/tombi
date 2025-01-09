use crate::{backend, hover::HoverContent, toml};
use ast::{algo::ancestors_at_position, AstNode};
use document_tree::TryIntoDocumentTree;
use itertools::Itertools;
use schema_store::get_keys_value_info;
use tower_lsp::lsp_types::{Hover, HoverParams, TextDocumentPositionParams};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_hover(
    backend: &backend::Backend,
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
    let position = position.into();
    let toml_version = backend.toml_version().await.unwrap_or_default();

    let Some(root) = ast::Root::cast(parser::parse(&source, toml_version).into_syntax_node())
    else {
        return Ok(None);
    };

    let Some((keys, range)) = get_hover_range(&root, position, toml_version) else {
        return Ok(None);
    };

    if keys.is_empty() {
        return Ok(None);
    }

    let Ok(root) = root.try_into_document_tree(toml_version) else {
        return Ok(None);
    };

    let Some(keys_value_info) = get_keys_value_info(root, &keys, position, toml_version) else {
        return Ok(None);
    };

    return Ok(Some(
        HoverContent {
            keys_value_info: Some(keys_value_info),
            range,
            ..Default::default()
        }
        .into(),
    ));
}

fn get_hover_range(
    root: &ast::Root,
    position: text::Position,
    toml_version: config::TomlVersion,
) -> Option<(Vec<document_tree::Key>, Option<text::Range>)> {
    let mut keys_vec = vec![];
    let mut hover_range = None;

    for node in ancestors_at_position(root.syntax(), position) {
        if let Some(array) = ast::Array::cast(node.to_owned()) {
            for (value, comma) in array.values_with_comma() {
                if hover_range.is_none() {
                    let mut range = value.range();
                    if let Some(comma) = comma {
                        range += comma.range()
                    };
                    if range.contains(position) {
                        hover_range = Some(range);
                    }
                }
            }
        };

        let keys = if let Some(kv) = ast::KeyValue::cast(node.to_owned()) {
            if hover_range.is_none() {
                if let Some(inline_table) = ast::InlineTable::cast(node.parent().unwrap()) {
                    for (key_value, comma) in inline_table.key_values_with_comma() {
                        if hover_range.is_none() {
                            let mut range = key_value.range();
                            if let Some(comma) = comma {
                                range += comma.range()
                            };
                            if range.contains(position) {
                                hover_range = Some(range);
                                break;
                            }
                        }
                    }
                } else {
                    hover_range = Some(kv.range());
                }
            }
            kv.keys().unwrap()
        } else if let Some(table) = ast::Table::cast(node.to_owned()) {
            table.header().unwrap()
        } else if let Some(array_of_tables) = ast::ArrayOfTables::cast(node.to_owned()) {
            array_of_tables.header().unwrap()
        } else {
            continue;
        };

        let keys = if keys.range().contains(position) {
            let mut new_keys = Vec::with_capacity(keys.keys().count());
            for key in keys
                .keys()
                .take_while(|key| key.token().unwrap().range().start() <= position)
            {
                match key.try_into_document_tree(toml_version) {
                    Ok(key) => new_keys.push(key),
                    Err(_) => return None,
                }
            }
            new_keys
        } else {
            let mut new_keys = Vec::with_capacity(keys.keys().count());
            for key in keys.keys() {
                match key.try_into_document_tree(toml_version) {
                    Ok(key) => new_keys.push(key),
                    Err(_) => return None,
                }
            }
            new_keys
        };

        if hover_range.is_none() {
            hover_range = keys.iter().map(|key| key.range()).reduce(|k1, k2| k1 + k2);
        }

        keys_vec.push(keys);
    }

    Some((
        keys_vec.into_iter().rev().flatten().collect_vec(),
        hover_range,
    ))
}
