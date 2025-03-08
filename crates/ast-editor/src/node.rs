use parser::parse_as;

pub fn make_comma(toml_version: toml_version::TomlVersion) -> syntax::SyntaxNode {
    parse_as::<ast::Comma>(&",", toml_version).into_syntax_node_mut()
}

pub fn make_comma_with_tailing_comment(
    tailing_comment: &ast::TailingComment,
    toml_version: toml_version::TomlVersion,
) -> syntax::SyntaxNode {
    parse_as::<ast::Comma>(
        &format!(",{}", tailing_comment.syntax().text()),
        toml_version,
    )
    .into_syntax_node_mut()
}
