use ast::{AstNode, AstToken};
use parser::{SyntaxNode, SyntaxToken};
use text::TextRange;
use tower_lsp::lsp_types::{
    Position, Range, SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensParams,
    SemanticTokensResult,
};

use crate::{server::backend::Backend, toml};

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum TokenType {
    STRUCT,
    STRING,
    NUMBER,
    KEYWORD,
    VARIABLE,
    REGEXP,
    OPERATOR,
    COMMENT,
}

impl TokenType {
    pub const LEGEND: &'static [SemanticTokenType] = &[
        SemanticTokenType::STRUCT,
        SemanticTokenType::STRING,
        SemanticTokenType::NUMBER,
        SemanticTokenType::KEYWORD,
        SemanticTokenType::VARIABLE,
        SemanticTokenType::REGEXP,
        SemanticTokenType::OPERATOR,
        SemanticTokenType::COMMENT,
    ];
}

pub async fn handle_semantic_tokens_full(
    _backend: &Backend,
    SemanticTokensParams { text_document, .. }: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>, tower_lsp::jsonrpc::Error> {
    tracing::info!("semantic_tokens_full");

    let source = toml::try_load(&text_document.uri)?;

    let p = parser::parse(&source);
    let Some(ast) = ast::Root::cast(p.into_syntax_node()) else {
        return Ok(None);
    };

    let mut tokens_builder = SemanticTokensBuilder::new(&source);
    ast.append_semantic_tokens(&mut tokens_builder);
    let tokens = tokens_builder.build();

    tracing::debug!("SemanticTokens: {tokens:#?}");

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: tokens,
    })))
}

trait AppendSemanticTokens {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder);
}

impl AppendSemanticTokens for ast::Root {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        self.begin_dangling_comments()
            .iter()
            .for_each(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));

        for item in self.items() {
            item.append_semantic_tokens(builder);
        }
        self.end_dangling_comments()
            .iter()
            .for_each(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));
    }
}

impl AppendSemanticTokens for ast::RootItem {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        match self {
            Self::Table(table) => table.append_semantic_tokens(builder),
            Self::ArrayOfTable(array) => array.append_semantic_tokens(builder),
            Self::KeyValue(key_value) => key_value.append_semantic_tokens(builder),
        }
    }
}

impl AppendSemanticTokens for ast::Table {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        self.header_leading_comments()
            .iter()
            .for_each(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));

        self.bracket_start()
            .map(|token| builder.add_token(TokenType::OPERATOR, (&token).into()));

        if let Some(header) = self.header() {
            for key in header.keys() {
                builder.add_token(TokenType::STRUCT, key.syntax().into());
            }
        }

        self.bracket_end()
            .map(|token| builder.add_token(TokenType::OPERATOR, (&token).into()));

        self.header_tailing_comment()
            .map(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));

        for key_value in self.key_values() {
            key_value.append_semantic_tokens(builder);
        }
    }
}

impl AppendSemanticTokens for ast::ArrayOfTable {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        self.header_leading_comments()
            .iter()
            .for_each(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));

        self.double_bracket_start().map(|token| {
            builder.add_token(TokenType::OPERATOR, (&token).into());
        });

        if let Some(header) = self.header() {
            for key in header.keys() {
                builder.add_token(TokenType::STRUCT, key.syntax().into());
            }
        }

        self.double_bracket_end().map(|token| {
            builder.add_token(TokenType::OPERATOR, (&token).into());
        });

        self.header_tailing_comment()
            .map(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));

        for key_value in self.key_values() {
            key_value.append_semantic_tokens(builder);
        }
    }
}

impl AppendSemanticTokens for ast::KeyValue {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        self.leading_comments()
            .iter()
            .for_each(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));

        self.keys().map(|key| key.append_semantic_tokens(builder));
        self.value()
            .map(|value| value.append_semantic_tokens(builder));
    }
}

impl AppendSemanticTokens for ast::Keys {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for key in self.keys() {
            key.append_semantic_tokens(builder);
        }
    }
}

impl AppendSemanticTokens for ast::Key {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        builder.add_token(TokenType::VARIABLE, self.syntax().into());
    }
}

impl AppendSemanticTokens for ast::Value {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        match self {
            Self::BasicString(n) => {
                builder.add_token(TokenType::STRING, (&n.token().unwrap()).into())
            }
            Self::LiteralString(n) => {
                builder.add_token(TokenType::STRING, (&n.token().unwrap()).into())
            }
            Self::MultiLineBasicString(n) => {
                builder.add_token(TokenType::STRING, (&n.token().unwrap()).into())
            }
            Self::MultiLineLiteralString(n) => {
                builder.add_token(TokenType::STRING, (&n.token().unwrap()).into())
            }
            Self::IntegerBin(n) => {
                builder.add_token(TokenType::NUMBER, (&n.token().unwrap()).into())
            }
            Self::IntegerOct(n) => {
                builder.add_token(TokenType::NUMBER, (&n.token().unwrap()).into())
            }
            Self::IntegerDec(n) => {
                builder.add_token(TokenType::NUMBER, (&n.token().unwrap()).into())
            }
            Self::IntegerHex(n) => {
                builder.add_token(TokenType::NUMBER, (&n.token().unwrap()).into())
            }
            Self::Float(n) => builder.add_token(TokenType::NUMBER, (&n.token().unwrap()).into()),
            Self::Boolean(n) => builder.add_token(TokenType::KEYWORD, (&n.token().unwrap()).into()),
            Self::OffsetDateTime(n) => {
                builder.add_token(TokenType::REGEXP, (&n.token().unwrap()).into())
            }
            Self::LocalDateTime(n) => {
                builder.add_token(TokenType::REGEXP, (&n.token().unwrap()).into())
            }
            Self::LocalDate(n) => {
                builder.add_token(TokenType::REGEXP, (&n.token().unwrap()).into())
            }
            Self::LocalTime(n) => {
                builder.add_token(TokenType::REGEXP, (&n.token().unwrap()).into())
            }
            Self::Array(array) => array.append_semantic_tokens(builder),
            Self::InlineTable(inline_table) => inline_table.append_semantic_tokens(builder),
        }

        self.tailing_comment()
            .map(|comment| builder.add_token(TokenType::COMMENT, comment.syntax().into()));
    }
}

impl AppendSemanticTokens for ast::Array {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for value in self.values() {
            value.append_semantic_tokens(builder);
        }
    }
}

impl AppendSemanticTokens for ast::InlineTable {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for entry in self.entries() {
            entry.append_semantic_tokens(builder);
        }
    }
}

struct SemanticTokensBuilder<'a> {
    tokens: Vec<SemanticToken>,
    last_range: Range,
    source: &'a str,
}

impl<'a> SemanticTokensBuilder<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            last_range: Range::default(),
            source,
        }
    }

    fn add_token(&mut self, token_type: TokenType, node: TokenOrNode) {
        let range = Range::new(
            text::Position::from_source(self.source, node.text_range().start()).into(),
            text::Position::from_source(self.source, node.text_range().end()).into(),
        );

        let relative = relative_range(range, self.last_range);

        #[allow(clippy::cast_possible_truncation)]
        self.tokens.push(SemanticToken {
            delta_line: relative.start.line as u32,
            delta_start: relative.start.character as u32,
            length: (relative.end.character - relative.start.character) as u32,
            token_type: token_type as u32,
            token_modifiers_bitset: 0,
        });

        self.last_range = range;
    }

    fn build(self) -> Vec<SemanticToken> {
        self.tokens
    }
}

enum TokenOrNode<'a> {
    Token(&'a SyntaxToken),
    Node(&'a SyntaxNode),
}

impl<'a> TokenOrNode<'a> {
    fn text_range(&self) -> TextRange {
        match self {
            Self::Token(token) => token.text_range(),
            Self::Node(node) => node.text_range(),
        }
    }
}

impl<'a> From<&'a SyntaxToken> for TokenOrNode<'a> {
    fn from(token: &'a SyntaxToken) -> Self {
        Self::Token(token)
    }
}

impl<'a> From<&'a SyntaxNode> for TokenOrNode<'a> {
    fn from(node: &'a SyntaxNode) -> Self {
        Self::Node(node)
    }
}

pub fn relative_position(position: Position, to: Position) -> Position {
    if position.line == to.line {
        Position {
            line: 0,
            character: position.character - to.character,
        }
    } else {
        Position {
            line: position.line - to.line,
            character: position.character,
        }
    }
}

pub fn relative_range(range: Range, to: Range) -> Range {
    let line_diff = range.end.line - range.start.line;
    let start = relative_position(range.start, to.start);

    let end = if line_diff == 0 {
        Position {
            line: start.line,
            character: start.character + range.end.character - range.start.character,
        }
    } else {
        Position {
            line: start.line + line_diff,
            character: range.end.character,
        }
    };

    Range { start, end }
}
