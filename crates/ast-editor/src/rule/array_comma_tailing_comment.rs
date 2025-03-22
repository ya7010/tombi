use ast::AstNode;
use itertools::Itertools;
use schema_store::SchemaContext;
use syntax::SyntaxElement;

use crate::{change::Change, node::make_comma_with_tailing_comment};

pub fn array_comma_tailing_comment(
    value: &ast::Value,
    comma: Option<&ast::Comma>,
    _schema_context: &SchemaContext,
) -> Vec<Change> {
    if let Some(tailing_comment) = value.tailing_comment() {
        if match comma {
            Some(comma) => {
                comma.tailing_comment().is_none()
                    && comma.leading_comments().collect_vec().is_empty()
            }
            None => true,
        } {
            let comma_with_tailing_comment = make_comma_with_tailing_comment(&tailing_comment);

            return vec![
                Change::Remove {
                    target: SyntaxElement::Token(tailing_comment.syntax().clone()),
                },
                Change::Append {
                    base: SyntaxElement::Node(value.syntax().clone()),
                    new: vec![SyntaxElement::Node(comma_with_tailing_comment)],
                },
            ];
        }
    }

    Vec::with_capacity(0)
}
