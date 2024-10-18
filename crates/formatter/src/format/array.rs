use crate::Format;

impl Format for ast::Array {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        let elements = self
            .elements()
            .map(|it| it.format(context))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[ {} ]", elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::AstNode;
    use rstest::rstest;

    #[rstest]
    #[case(r#"integers = [ 1, 2, 3 ]"#)]
    #[case(r#"colors = [ "red", "yellow", "green" ]"#)]
    #[case(r#"nested_arrays_of_ints = [ [ 1, 2 ], [ 3, 4, 5 ] ]"#)]
    #[case(r#"nested_mixed_array = [ [ 1, 2 ], [ "a", "b", "c" ] ]"#)]
    #[case(r#"string_array = [ "all", 'strings', """are the same""", '''type''' ]"#)]
    fn single_line_array(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        assert_eq!(p.errors(), []);
        assert_eq!(ast.format_default(), source);
    }
}
