#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Tag {
    Key,
    TableKey,
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,
    DateTimeLiteral,
    DateLiteral,
    TimeLiteral,
    /// =
    Equal,
    /// []
    Bracket,
    /// {}
    Brace,
    /// ,
    Comma,
    /// .
    Dot,
    /// # comment
    Comment,
    // For things which don't have a specific highlight.
    None,
}

impl Tag {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Key => "key",
            Self::TableKey => "table_key",
            Self::StringLiteral => "string",
            Self::IntegerLiteral => "integer",
            Self::FloatLiteral => "float",
            Self::BooleanLiteral => "boolean",
            Self::DateTimeLiteral => "datetime",
            Self::DateLiteral => "date",
            Self::TimeLiteral => "time",
            Self::Equal => "equal",
            Self::Bracket => "bracket",
            Self::Brace => "brace",
            Self::Comma => "comma",
            Self::Dot => "dot",
            Self::Comment => "comment",
            Self::None => "none",
        }
    }
}
