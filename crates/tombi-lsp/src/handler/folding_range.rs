use itertools::Itertools;
use tombi_ast::AstNode;
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_folding_range(
    backend: &Backend,
    params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_folding_range");
    tracing::trace!(?params);

    let FoldingRangeParams { text_document, .. } = params;

    let Some(Ok(root)) = backend.try_get_ast(&text_document.uri).await else {
        return Ok(None);
    };

    let folding_ranges = create_folding_ranges(root);

    if !folding_ranges.is_empty() {
        Ok(Some(folding_ranges))
    } else {
        Ok(None)
    }
}

fn create_folding_ranges(root: tombi_ast::Root) -> Vec<FoldingRange> {
    let mut ranges: Vec<FoldingRange> = vec![];

    for node in root.syntax().descendants() {
        if let Some(table) = tombi_ast::Table::cast(node.to_owned()) {
            for folding_range in [table.get_folding_range()] {
                if let Some(folding_range) = folding_range {
                    ranges.push(FoldingRange {
                        start_line: folding_range.start().line(),
                        start_character: Some(folding_range.start().column()),
                        end_line: folding_range.end().line(),
                        end_character: Some(folding_range.end().column()),
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: None,
                    });
                }
            }
        } else if let Some(array_of_table) = tombi_ast::ArrayOfTable::cast(node.to_owned()) {
            for folding_range in [array_of_table.get_folding_range()] {
                if let Some(folding_range) = folding_range {
                    ranges.push(FoldingRange {
                        start_line: folding_range.start().line(),
                        start_character: Some(folding_range.start().column()),
                        end_line: folding_range.end().line(),
                        end_character: Some(folding_range.end().column()),
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: None,
                    });
                }
            }
        } else if let Some(array) = tombi_ast::Array::cast(node.to_owned()) {
            let start_position = array.bracket_start().unwrap().range().start();
            let end_position = array.bracket_end().unwrap().range().end();

            ranges.push(FoldingRange {
                start_line: start_position.line(),
                start_character: Some(start_position.column()),
                end_line: end_position.line(),
                end_character: Some(end_position.column()),
                kind: Some(FoldingRangeKind::Region),
                collapsed_text: None,
            });
        } else if let Some(inline_table) = tombi_ast::InlineTable::cast(node.to_owned()) {
            let start_position = inline_table.brace_start().unwrap().range().start();
            let end_position = inline_table.brace_end().unwrap().range().end();

            ranges.push(FoldingRange {
                start_line: start_position.line(),
                start_character: Some(start_position.column()),
                end_line: end_position.line(),
                end_character: Some(end_position.column()),
                kind: Some(FoldingRangeKind::Region),
                collapsed_text: None,
            });
        }
    }

    ranges
}

trait GetFoldingRange {
    fn get_folding_range(&self) -> Option<tombi_text::Range>;
}

impl GetFoldingRange for tombi_ast::Table {
    fn get_folding_range(&self) -> Option<tombi_text::Range> {
        use tombi_syntax::{SyntaxKind::*, T};

        let children_with_tokens = self.syntax().children_with_tokens().collect_vec();
        let first_child = children_with_tokens
            .iter()
            .find(|child| matches!(child.kind(), T!('[')));
        let last_child = children_with_tokens
            .iter()
            .rev()
            .find(|child| !matches!(child.kind(), WHITESPACE | LINE_BREAK));

        match (first_child, last_child) {
            (Some(first), Some(last)) => Some(tombi_text::Range::new(
                first.range().start(),
                self.subtables()
                    .last()
                    .and_then(|t| t.get_folding_range())
                    .unwrap_or(last.range())
                    .end(),
            )),
            _ => None,
        }
    }
}

impl GetFoldingRange for tombi_ast::ArrayOfTable {
    fn get_folding_range(&self) -> Option<tombi_text::Range> {
        use tombi_syntax::{SyntaxKind::*, T};

        let children_with_tokens = self.syntax().children_with_tokens().collect_vec();
        let first_child = children_with_tokens
            .iter()
            .find(|child| matches!(child.kind(), T!("[[")));
        let last_child = children_with_tokens
            .iter()
            .rev()
            .find(|child| !matches!(child.kind(), WHITESPACE | LINE_BREAK));

        match (first_child, last_child) {
            (Some(first), Some(last)) => Some(tombi_text::Range::new(
                first.range().start(),
                self.subtables()
                    .last()
                    .and_then(|t| t.get_folding_range())
                    .unwrap_or(last.range())
                    .end(),
            )),
            _ => None,
        }
    }
}

impl GetFoldingRange for tombi_ast::TableOrArrayOfTable {
    fn get_folding_range(&self) -> Option<tombi_text::Range> {
        match self {
            Self::Table(table) => table.get_folding_range(),
            Self::ArrayOfTable(array_of_table) => array_of_table.get_folding_range(),
        }
    }
}

impl GetFoldingRange for Vec<tombi_ast::LeadingComment> {
    fn get_folding_range(&self) -> Option<tombi_text::Range> {
        let first = self.first()?;
        let last = self.last()?;
        Some(tombi_text::Range::new(
            first.syntax().range().start(),
            last.syntax().range().end(),
        ))
    }
}

impl GetFoldingRange for Vec<tombi_ast::BeginDanglingComment> {
    fn get_folding_range(&self) -> Option<tombi_text::Range> {
        let first = self.first()?;
        let last = self.last()?;
        Some(tombi_text::Range::new(
            first.syntax().range().start(),
            last.syntax().range().end(),
        ))
    }
}
