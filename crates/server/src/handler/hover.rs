use crate::{backend, hover::HoverContent, toml};
use ast::{algo::ancestors_at_position, AstNode};
use itertools::Itertools;
use json_schema_store::{get_accessors, Accessors};
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

    let Some(root) =
        ast::Root::cast(parser::parse(&source, backend.toml_version()).into_syntax_node())
    else {
        return Ok(None);
    };

    let keys = get_keys(&root, position);

    if keys.is_empty() {
        return Ok(None);
    }

    let Ok(root) = document_tree::Root::try_from(root) else {
        return Ok(None);
    };

    let accessors = get_accessors(root, &keys, position);

    if !accessors.is_empty() {
        let hover_content = HoverContent {
            accessor: Accessors::new(accessors),
            ..Default::default()
        };
        return Ok(Some(hover_content.into()));
    } else {
        return Ok(None);
    }
}

fn get_keys(root: &ast::Root, position: text::Position) -> Vec<document_tree::Key> {
    let mut keys_vec = vec![];
    for node in ancestors_at_position(root.syntax(), position) {
        let keys = if let Some(kv) = ast::KeyValue::cast(node.to_owned()) {
            kv.keys().unwrap()
        } else if let Some(table) = ast::Table::cast(node.to_owned()) {
            table.header().unwrap()
        } else if let Some(array_of_tables) = ast::ArrayOfTables::cast(node.to_owned()) {
            array_of_tables.header().unwrap()
        } else {
            continue;
        };

        if keys.range().contains(position) {
            keys_vec.push(
                keys.keys()
                    .take_while(|key| key.token().unwrap().range().start() <= position)
                    .map(document_tree::Key::from)
                    .collect_vec(),
            )
        } else {
            keys_vec.push(keys.keys().map(document_tree::Key::from).collect_vec())
        }
    }
    keys_vec.into_iter().rev().flatten().collect_vec()
}
