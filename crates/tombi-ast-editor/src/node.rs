use tombi_parser::parse_as;

pub fn make_comma(toml_version: tombi_toml_version::TomlVersion) -> tombi_syntax::SyntaxNode {
    parse_as::<tombi_ast::Comma>(",", Some(toml_version)).into_syntax_node_mut()
}

pub fn make_comma_with_tailing_comment(
    tailing_comment: &tombi_ast::TailingComment,
    toml_version: tombi_toml_version::TomlVersion,
) -> tombi_syntax::SyntaxNode {
    parse_as::<tombi_ast::Comma>(
        &format!(",{}", tailing_comment.syntax().text()),
        Some(toml_version),
    )
    .into_syntax_node_mut()
}
