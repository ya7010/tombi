use crate::backend::Backend;
use ast::AstNode;
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_folding_range(
    backend: &Backend,
    FoldingRangeParams { text_document, .. }: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_folding_range");

    let Some(document) = backend.documents.get(&text_document.uri) else {
        return Ok(None);
    };

    let p = parser::parse(&document.source, backend.toml_version());
    if !p.errors().is_empty() {
        return Ok(None);
    }

    let Some(root) = ast::Root::cast(p.into_syntax_node()) else {
        return Ok(None);
    };

    let folding_ranges = create_folding_ranges(root);

    dbg!(&folding_ranges);
    if !folding_ranges.is_empty() {
        Ok(Some(folding_ranges))
    } else {
        Ok(None)
    }
}

fn create_folding_ranges(root: ast::Root) -> Vec<FoldingRange> {
    let mut ranges: Vec<FoldingRange> = vec![];

    for node in root.syntax().descendants() {
        if let Some(table) = ast::Table::cast(node.to_owned()) {
            let start_position = table.header().unwrap().range().start();
            let end_position = table
                .key_values()
                .last()
                .map_or(start_position, |last| last.syntax().text_range().end());

            ranges.push(FoldingRange {
                start_line: start_position.line(),
                start_character: None,
                end_line: end_position.line(),
                end_character: None,
                kind: Some(FoldingRangeKind::Region),
                collapsed_text: None,
            });
        } else if let Some(array_of_table) = ast::ArrayOfTable::cast(node.to_owned()) {
            let start_position = array_of_table.header().unwrap().range().start();
            let end_position = array_of_table
                .key_values()
                .last()
                .map_or(start_position, |last| last.syntax().text_range().end());

            ranges.push(FoldingRange {
                start_line: start_position.line(),
                start_character: None,
                end_line: end_position.line(),
                end_character: None,
                kind: Some(FoldingRangeKind::Region),
                collapsed_text: None,
            });
        }
    }

    ranges
}
