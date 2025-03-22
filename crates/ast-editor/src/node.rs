use parser::parse_as;

pub fn make_comma() -> syntax::SyntaxNode {
    parse_as::<ast::Comma>(",").into_syntax_node_mut()
}

pub fn make_comma_with_tailing_comment(
    tailing_comment: &ast::TailingComment,
) -> syntax::SyntaxNode {
    parse_as::<ast::Comma>(&format!(",{}", tailing_comment.syntax().text())).into_syntax_node_mut()
}
