use crate::{AstNode, KeyValue, Value};

#[derive(Debug, Clone)]
pub enum ValueOrKeyValue {
    Value(Value),
    KeyValue(KeyValue),
}

impl ValueOrKeyValue {
    pub fn span(&self) -> tombi_text::Span {
        match self {
            ValueOrKeyValue::Value(value) => value.span(),
            ValueOrKeyValue::KeyValue(key) => key.syntax().span(),
        }
    }
}

impl AstNode for ValueOrKeyValue {
    #[inline]
    fn can_cast(kind: tombi_ast_syntax::SyntaxKind) -> bool {
        Value::can_cast(kind) || KeyValue::can_cast(kind)
    }

    #[inline]
    fn cast(syntax: tombi_ast_syntax::SyntaxNode) -> Option<Self> {
        if Value::can_cast(syntax.kind()) {
            Value::cast(syntax).map(ValueOrKeyValue::Value)
        } else if KeyValue::can_cast(syntax.kind()) {
            Some(ValueOrKeyValue::KeyValue(KeyValue { syntax }))
        } else {
            None
        }
    }

    #[inline]
    fn syntax(&self) -> &tombi_ast_syntax::SyntaxNode {
        match self {
            ValueOrKeyValue::Value(value) => value.syntax(),
            ValueOrKeyValue::KeyValue(key) => key.syntax(),
        }
    }
}
