/// Definitions provides the definition of the format that does not have the freedom set by Options.
///
/// NOTE: Some of the items defined in Definitions may be moved to Options in the future.
#[derive(Debug, Clone, Copy)]
pub struct Definitions;

impl Definitions {
    pub const fn tailing_comment_space(&self) -> &'static str {
        "  "
    }

    pub const fn array_bracket_inner_space(&self) -> &'static str {
        ""
    }

    pub const fn inline_table_brace_inner_space(&self) -> &'static str {
        " "
    }
}
