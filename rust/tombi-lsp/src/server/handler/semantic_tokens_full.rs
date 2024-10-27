use ast::AstNode;
use parser::SyntaxNode;
use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensParams, SemanticTokensResult,
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
        SemanticTokenType::COMMENT,
    ];
}

pub async fn handle_semantic_tokens_full(
    backend: &Backend,
    SemanticTokensParams { text_document, .. }: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>, tower_lsp::jsonrpc::Error> {
    let source = toml::try_load(&text_document.uri)?;

    let p = parser::parse(&source);
    let Some(ast) = ast::Root::cast(p.into_syntax_node()) else {
        return Ok(None);
    };

    let mut tokens = Vec::new();
    ast.append_semantic_tokens(&mut tokens);

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: tokens,
    })))
}

trait AppendSemanticTokens {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>);
}

impl AppendSemanticTokens for ast::Root {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        for item in self.items() {
            item.append_semantic_tokens(tokens);
        }
    }
}

impl AppendSemanticTokens for ast::RootItem {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        match self {
            Self::Table(table) => table.append_semantic_tokens(tokens),
            Self::ArrayOfTable(array) => array.append_semantic_tokens(tokens),
            Self::KeyValue(key_value) => key_value.append_semantic_tokens(tokens),
        }
    }
}

impl AppendSemanticTokens for ast::Table {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        if let Some(header) = self.header() {
            for key in header.keys() {
                tokens.push(semantic_token(TokenType::STRUCT, key.syntax()));
            }
        }

        for entry in self.key_values() {
            entry.append_semantic_tokens(tokens);
        }
    }
}

impl AppendSemanticTokens for ast::ArrayOfTable {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        if let Some(header) = self.header() {
            for key in header.keys() {
                tokens.push(semantic_token(TokenType::STRUCT, key.syntax()));
            }
        }

        for table in self.key_values() {
            table.append_semantic_tokens(tokens);
        }
    }
}

impl AppendSemanticTokens for ast::KeyValue {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        self.keys().map(|key| key.append_semantic_tokens(tokens));
        self.value()
            .map(|value| value.append_semantic_tokens(tokens));
    }
}

impl AppendSemanticTokens for ast::Keys {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        for key in self.keys() {
            key.append_semantic_tokens(tokens);
        }
    }
}

impl AppendSemanticTokens for ast::Key {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        tokens.push(semantic_token(TokenType::VARIABLE, self.syntax()));
    }
}

impl AppendSemanticTokens for ast::Value {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        match self {
            Self::BasicString(_)
            | Self::LiteralString(_)
            | Self::MultiLineBasicString(_)
            | Self::MultiLineLiteralString(_) => {
                tokens.push(semantic_token(TokenType::STRING, self.syntax()))
            }
            Self::IntegerBin(_)
            | Self::IntegerOct(_)
            | Self::IntegerDec(_)
            | Self::IntegerHex(_)
            | Self::Float(_) => tokens.push(semantic_token(TokenType::NUMBER, self.syntax())),
            Self::Boolean(_) => tokens.push(semantic_token(TokenType::KEYWORD, self.syntax())),
            Self::OffsetDateTime(_)
            | Self::LocalDateTime(_)
            | Self::LocalDate(_)
            | Self::LocalTime(_) => tokens.push(semantic_token(TokenType::REGEXP, self.syntax())),
            Self::Array(array) => array.append_semantic_tokens(tokens),
            Self::InlineTable(inline_table) => inline_table.append_semantic_tokens(tokens),
        }
    }
}

impl AppendSemanticTokens for ast::Array {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        for value in self.elements() {
            value.append_semantic_tokens(tokens);
        }
    }
}

impl AppendSemanticTokens for ast::InlineTable {
    fn append_semantic_tokens(&self, tokens: &mut Vec<SemanticToken>) {
        for entry in self.elements() {
            entry.append_semantic_tokens(tokens);
        }
    }
}

fn semantic_token(token_type: TokenType, node: &SyntaxNode) -> SemanticToken {
    SemanticToken {
        delta_line: 0,
        delta_start: node.text_range().start().into(),
        length: node.text().len().into(),
        token_type: token_type as u32,
        token_modifiers_bitset: TokenType::LEGEND.iter().enumerate().fold(
            0,
            |mut total, (i, _)| {
                total += 1 << i;
                total
            },
        ),
    }
}
