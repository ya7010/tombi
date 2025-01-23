use ast::AstNode;
use toml_version::TomlVersion;

use crate::{DocumentTreeResult, IntoDocumentTreeResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyKind {
    BareKey,
    BasicString,
    LiteralString,
}

#[derive(Debug, Clone)]
pub struct Key {
    kind: KeyKind,
    value: String,
    range: text::Range,
}

impl Key {
    pub fn try_new(
        kind: KeyKind,
        value: String,
        range: text::Range,
        toml_version: TomlVersion,
    ) -> Result<Self, crate::Error> {
        let key = Self { kind, value, range };
        key.try_to_raw_string(toml_version)?;

        Ok(key)
    }
    pub fn kind(&self) -> KeyKind {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn to_raw_text(&self, toml_version: TomlVersion) -> String {
        // NOTE: Key has already been validated by `impl TryIntoDocumentTree<Key>`,
        //       so it's safe to unwrap.
        self.try_to_raw_string(toml_version).unwrap()
    }

    fn try_to_raw_string(
        &self,
        toml_version: TomlVersion,
    ) -> Result<std::string::String, crate::Error> {
        match self.kind {
            KeyKind::BareKey => {
                crate::support::string::try_from_bare_key(&self.value, toml_version)
            }
            KeyKind::BasicString => {
                crate::support::string::try_from_basic_string(&self.value, toml_version)
            }
            KeyKind::LiteralString => crate::support::string::try_from_literal_string(&self.value),
        }
        .map_err(|error| crate::Error::ParseStringError {
            error,
            range: self.range,
        })
    }

    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.try_to_raw_string(TomlVersion::latest())
            == other.try_to_raw_string(TomlVersion::latest())
    }
}

impl Eq for Key {}

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.try_to_raw_string(TomlVersion::latest())
            .unwrap_or_else(|_| self.value.to_string())
            .hash(state);
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl IntoDocumentTreeResult<Option<Key>> for ast::Key {
    fn into_document_tree_result(
        self,
        toml_version: TomlVersion,
    ) -> crate::DocumentTreeResult<Option<Key>> {
        let range = self.syntax().range();
        let Some(token) = self.token() else {
            return DocumentTreeResult {
                tree: None,
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match Key::try_new(
            match self {
                ast::Key::BareKey(_) => KeyKind::BareKey,
                ast::Key::BasicString(_) => KeyKind::BasicString,
                ast::Key::LiteralString(_) => KeyKind::LiteralString,
            },
            token.text().to_string(),
            token.range(),
            toml_version,
        ) {
            Ok(key) => DocumentTreeResult {
                tree: Some(key),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeResult {
                tree: None,
                errors: vec![error],
            },
        }
    }
}

impl IntoDocumentTreeResult<Vec<crate::Key>> for ast::Keys {
    fn into_document_tree_result(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> DocumentTreeResult<Vec<crate::Key>> {
        let mut keys = Vec::new();
        let mut errors = Vec::new();

        for key in self.keys() {
            let result = key.into_document_tree_result(toml_version);
            if !result.errors.is_empty() {
                errors.extend(result.errors);

                return DocumentTreeResult { tree: keys, errors };
            }
            if let Some(key) = result.tree {
                keys.push(key);
            }
        }

        DocumentTreeResult { tree: keys, errors }
    }
}
