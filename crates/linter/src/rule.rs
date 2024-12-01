mod inline_table_last_comma;
mod key_empty;
pub use key_empty::KeyEmptyRule;

pub trait Rule<N: ast::AstNode> {
    fn check(node: &N, l: &mut crate::Linter);
}
