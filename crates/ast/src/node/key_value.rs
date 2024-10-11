pub enum Key {
    BareKey,
    QuotedKey,
    DottedKeys,
}

pub enum Value {
    String,
    Integer,
    Float,
    Boolean,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,
    Array,
    InlineTable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyValue {
    pub(crate) syntax: syntax::SyntaxNode,
}

impl KeyValue {
    pub fn key(&self) -> Option<Key> {
        crate::support::child(self.syntax())
    }

    pub fn eq_token(&self) -> Option<syntax::SyntaxToken> {
        crate::support::token(self.syntax(), syntax::SyntaxKind::EQUAL)
    }

    pub fn value(&self) -> Option<Value> {
        crate::support::child(self.syntax())
    }
}
