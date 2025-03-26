use ast::AstNode;
use itertools::Itertools;
use syntax::SyntaxElement;

use crate::{change::Change, node::make_comma_with_tailing_comment};

pub fn inline_table_comma_tailing_comment(
    key_value: &ast::KeyValue,
    comma: Option<&ast::Comma>,
) -> Vec<Change> {
    if let Some(tailing_comment) = key_value.tailing_comment() {
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
                    base: SyntaxElement::Node(key_value.syntax().clone()),
                    new: vec![SyntaxElement::Node(comma_with_tailing_comment)],
                },
            ];
        }
    }

    Vec::with_capacity(0)
}
