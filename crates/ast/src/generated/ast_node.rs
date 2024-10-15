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
    pub fn bracket_start_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    #[inline]
    pub fn bracket_end_token(&self) -> Option<SyntaxToken> {
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
    pub fn key_values(&self) -> AstChildren<KeyValue> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn double_bracket_start_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["[["])
    }
    #[inline]
    pub fn double_bracket_end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["]]"])
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
    pub fn brace_start_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    #[inline]
    pub fn brace_end_token(&self) -> Option<SyntaxToken> {
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
    pub fn integer_dec_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![integer_dec])
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
    pub fn value(&self) -> Option<Value> {
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
impl Root {
    #[inline]
    pub fn items(&self) -> AstChildren<RootItem> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String {
    pub(crate) syntax: SyntaxNode,
}
impl String {
    #[inline]
    pub fn basic_string_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![basic_string])
    }
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
    pub fn key_values(&self) -> AstChildren<KeyValue> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn bracket_start_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    #[inline]
    pub fn bracket_end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DottedKey {
    BareKey(BareKey),
    QuotedKey(QuotedKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    BareKey(BareKey),
    DottedKeys(DottedKeys),
    QuotedKey(QuotedKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RootItem {
    ArrayOfTable(ArrayOfTable),
    KeyValue(KeyValue),
    Table(Table),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Array(Array),
    Boolean(Boolean),
    Float(Float),
    InlineTable(InlineTable),
    Integer(Integer),
    LocalDate(LocalDate),
    LocalDateTime(LocalDateTime),
    LocalTime(LocalTime),
    OffsetDateTime(OffsetDateTime),
    String(String),
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
impl From<BareKey> for DottedKey {
    #[inline]
    fn from(node: BareKey) -> DottedKey {
        DottedKey::BareKey(node)
    }
}
impl From<QuotedKey> for DottedKey {
    #[inline]
    fn from(node: QuotedKey) -> DottedKey {
        DottedKey::QuotedKey(node)
    }
}
impl AstNode for DottedKey {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::BARE_KEY | SyntaxKind::QUOTED_KEY)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::BARE_KEY => DottedKey::BareKey(BareKey { syntax }),
            SyntaxKind::QUOTED_KEY => DottedKey::QuotedKey(QuotedKey { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            DottedKey::BareKey(it) => &it.syntax,
            DottedKey::QuotedKey(it) => &it.syntax,
        }
    }
}
impl From<BareKey> for Key {
    #[inline]
    fn from(node: BareKey) -> Key {
        Key::BareKey(node)
    }
}
impl From<DottedKeys> for Key {
    #[inline]
    fn from(node: DottedKeys) -> Key {
        Key::DottedKeys(node)
    }
}
impl From<QuotedKey> for Key {
    #[inline]
    fn from(node: QuotedKey) -> Key {
        Key::QuotedKey(node)
    }
}
impl AstNode for Key {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::BARE_KEY | SyntaxKind::DOTTED_KEYS | SyntaxKind::QUOTED_KEY
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::BARE_KEY => Key::BareKey(BareKey { syntax }),
            SyntaxKind::DOTTED_KEYS => Key::DottedKeys(DottedKeys { syntax }),
            SyntaxKind::QUOTED_KEY => Key::QuotedKey(QuotedKey { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Key::BareKey(it) => &it.syntax,
            Key::DottedKeys(it) => &it.syntax,
            Key::QuotedKey(it) => &it.syntax,
        }
    }
}
impl From<ArrayOfTable> for RootItem {
    #[inline]
    fn from(node: ArrayOfTable) -> RootItem {
        RootItem::ArrayOfTable(node)
    }
}
impl From<KeyValue> for RootItem {
    #[inline]
    fn from(node: KeyValue) -> RootItem {
        RootItem::KeyValue(node)
    }
}
impl From<Table> for RootItem {
    #[inline]
    fn from(node: Table) -> RootItem {
        RootItem::Table(node)
    }
}
impl AstNode for RootItem {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ARRAY_OF_TABLE | SyntaxKind::KEY_VALUE | SyntaxKind::TABLE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ARRAY_OF_TABLE => RootItem::ArrayOfTable(ArrayOfTable { syntax }),
            SyntaxKind::KEY_VALUE => RootItem::KeyValue(KeyValue { syntax }),
            SyntaxKind::TABLE => RootItem::Table(Table { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            RootItem::ArrayOfTable(it) => &it.syntax,
            RootItem::KeyValue(it) => &it.syntax,
            RootItem::Table(it) => &it.syntax,
        }
    }
}
impl From<Array> for Value {
    #[inline]
    fn from(node: Array) -> Value {
        Value::Array(node)
    }
}
impl From<Boolean> for Value {
    #[inline]
    fn from(node: Boolean) -> Value {
        Value::Boolean(node)
    }
}
impl From<Float> for Value {
    #[inline]
    fn from(node: Float) -> Value {
        Value::Float(node)
    }
}
impl From<InlineTable> for Value {
    #[inline]
    fn from(node: InlineTable) -> Value {
        Value::InlineTable(node)
    }
}
impl From<Integer> for Value {
    #[inline]
    fn from(node: Integer) -> Value {
        Value::Integer(node)
    }
}
impl From<LocalDate> for Value {
    #[inline]
    fn from(node: LocalDate) -> Value {
        Value::LocalDate(node)
    }
}
impl From<LocalDateTime> for Value {
    #[inline]
    fn from(node: LocalDateTime) -> Value {
        Value::LocalDateTime(node)
    }
}
impl From<LocalTime> for Value {
    #[inline]
    fn from(node: LocalTime) -> Value {
        Value::LocalTime(node)
    }
}
impl From<OffsetDateTime> for Value {
    #[inline]
    fn from(node: OffsetDateTime) -> Value {
        Value::OffsetDateTime(node)
    }
}
impl From<String> for Value {
    #[inline]
    fn from(node: String) -> Value {
        Value::String(node)
    }
}
impl AstNode for Value {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ARRAY
                | SyntaxKind::BOOLEAN
                | SyntaxKind::FLOAT
                | SyntaxKind::INLINE_TABLE
                | SyntaxKind::INTEGER
                | SyntaxKind::LOCAL_DATE
                | SyntaxKind::LOCAL_DATE_TIME
                | SyntaxKind::LOCAL_TIME
                | SyntaxKind::OFFSET_DATE_TIME
                | SyntaxKind::STRING
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ARRAY => Value::Array(Array { syntax }),
            SyntaxKind::BOOLEAN => Value::Boolean(Boolean { syntax }),
            SyntaxKind::FLOAT => Value::Float(Float { syntax }),
            SyntaxKind::INLINE_TABLE => Value::InlineTable(InlineTable { syntax }),
            SyntaxKind::INTEGER => Value::Integer(Integer { syntax }),
            SyntaxKind::LOCAL_DATE => Value::LocalDate(LocalDate { syntax }),
            SyntaxKind::LOCAL_DATE_TIME => Value::LocalDateTime(LocalDateTime { syntax }),
            SyntaxKind::LOCAL_TIME => Value::LocalTime(LocalTime { syntax }),
            SyntaxKind::OFFSET_DATE_TIME => Value::OffsetDateTime(OffsetDateTime { syntax }),
            SyntaxKind::STRING => Value::String(String { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Value::Array(it) => &it.syntax,
            Value::Boolean(it) => &it.syntax,
            Value::Float(it) => &it.syntax,
            Value::InlineTable(it) => &it.syntax,
            Value::Integer(it) => &it.syntax,
            Value::LocalDate(it) => &it.syntax,
            Value::LocalDateTime(it) => &it.syntax,
            Value::LocalTime(it) => &it.syntax,
            Value::OffsetDateTime(it) => &it.syntax,
            Value::String(it) => &it.syntax,
        }
    }
}
impl std::fmt::Display for DottedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RootItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ArrayOfTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BareKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DottedKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for InlineTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for KeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LocalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LocalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for QuotedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
