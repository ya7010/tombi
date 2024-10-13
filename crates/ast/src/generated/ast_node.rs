//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::support;
use crate::AstChildren;
use crate::AstNode;
use syntax::{SyntaxKind, SyntaxNode, SyntaxToken, T};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array {
    pub(crate) syntax: SyntaxNode,
}
impl Array {
    #[inline]
    pub fn elements(&self) -> AstChildren<Value> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn array_open_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    #[inline]
    pub fn array_close_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayOfTable {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayOfTable {
    #[inline]
    pub fn header(&self) -> Option<Key> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn array_table_open_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![T!["[["]])
    }
    #[inline]
    pub fn array_table_close_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![T!["]]"]])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BareKey {
    pub(crate) syntax: SyntaxNode,
}
impl BareKey {
    #[inline]
    pub fn bare_key_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![bare_key])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boolean {
    pub(crate) syntax: SyntaxNode,
}
impl Boolean {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DottedKeys {
    pub(crate) syntax: SyntaxNode,
}
impl DottedKeys {
    #[inline]
    pub fn dotted_keys(&self) -> AstChildren<DottedKey> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Float {
    pub(crate) syntax: SyntaxNode,
}
impl Float {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InlineTable {
    pub(crate) syntax: SyntaxNode,
}
impl InlineTable {
    #[inline]
    pub fn elements(&self) -> AstChildren<KeyValue> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn inline_table_open_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    #[inline]
    pub fn inline_table_close_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Integer {
    pub(crate) syntax: SyntaxNode,
}
impl Integer {
    #[inline]
    pub fn integer_bin_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![integer_bin])
    }
    #[inline]
    pub fn integer_hex_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![integer_hex])
    }
    #[inline]
    pub fn integer_oct_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![integer_oct])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyValue {
    pub(crate) syntax: SyntaxNode,
}
impl KeyValue {
    #[inline]
    pub fn key(&self) -> Option<Key> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [=])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalDate {
    pub(crate) syntax: SyntaxNode,
}
impl LocalDate {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalDateTime {
    pub(crate) syntax: SyntaxNode,
}
impl LocalDateTime {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalTime {
    pub(crate) syntax: SyntaxNode,
}
impl LocalTime {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OffsetDateTime {
    pub(crate) syntax: SyntaxNode,
}
impl OffsetDateTime {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuotedKey {
    pub(crate) syntax: SyntaxNode,
}
impl QuotedKey {
    #[inline]
    pub fn basic_string_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![basic_string])
    }
    #[inline]
    pub fn literal_string_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![literal_string])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Root {
    pub(crate) syntax: SyntaxNode,
}
impl Root {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String {
    pub(crate) syntax: SyntaxNode,
}
impl String {
    #[inline]
    pub fn literal_string_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![literal_string])
    }
    #[inline]
    pub fn multi_line_basic_string_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![multi_line_basic_string])
    }
    #[inline]
    pub fn multi_line_literal_string_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![multi_line_literal_string])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table {
    pub(crate) syntax: SyntaxNode,
}
impl Table {
    #[inline]
    pub fn header(&self) -> Option<Key> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn array_open_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    #[inline]
    pub fn array_close_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}
impl AstNode for Array {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARRAY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ArrayOfTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARRAY_OF_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BareKey {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BARE_KEY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Boolean {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BOOLEAN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DottedKeys {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DOTTED_KEYS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Float {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FLOAT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for InlineTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INLINE_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Integer {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INTEGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for KeyValue {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::KEY_VALUE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LocalDate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOCAL_DATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LocalDateTime {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOCAL_DATE_TIME
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LocalTime {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOCAL_TIME
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OffsetDateTime {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OFFSET_DATE_TIME
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for QuotedKey {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::QUOTED_KEY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Root {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ROOT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for String {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRING
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Table {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
