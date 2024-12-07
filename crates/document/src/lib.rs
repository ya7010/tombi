mod key;
mod value;

use std::ops::Deref;

pub use key::Key;
pub use value::{
    Array, ArrayKind, Boolean, Float, Integer, IntegerKind, LocalDate, LocalDateTime, LocalTime,
    OffsetDateTime, String, Table, TableKind, Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Document(Table);

impl From<Document> for Table {
    fn from(document: Document) -> Self {
        document.0
    }
}

impl From<document_tree::DocumentTree> for Document {
    fn from(document: document_tree::DocumentTree) -> Self {
        Self(document_tree::Table::from(document).into())
    }
}

impl Deref for Document {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_serialize {
    {#[test] fn $name:ident($source:expr) -> Ok($json:expr)} => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use ast::AstNode;

            let p = parser::parse($source, config::TomlVersion::V1_0_0);
            let ast = ast::Root::cast(p.into_syntax_node()).unwrap();
            let document_tree = document_tree::DocumentTree::try_from(ast).unwrap();
            let document: crate::Document = document_tree.into();
            let serialized = serde_json::to_string(&document).unwrap();
            pretty_assertions::assert_eq!(serialized, $json.to_string());
        }
    };
}

#[cfg(test)]
mod test {
    use serde_json::json;

    test_serialize! {
        #[test]
        fn empty("") -> Ok(json!({}))
    }

    test_serialize! {
        #[test]
        fn key_values(
            r#"
            key = "value"
            flag = true
            "#
        ) -> Ok(json!({"key": "value", "flag": true}))
    }
}
