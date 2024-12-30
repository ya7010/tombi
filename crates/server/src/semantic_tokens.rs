mod builder;
mod token_type;

use ast::{AstNode, AstToken};
pub use builder::SemanticTokensBuilder;
pub use token_type::{TokenType, SUPPORTED_TOKEN_TYPES};

pub trait AppendSemanticTokens {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder);
}

impl AppendSemanticTokens for ast::Root {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for comments in self.begin_dangling_comments() {
            for comment in comments {
                builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
            }
        }

        for item in self.items() {
            item.append_semantic_tokens(builder);
        }

        for comments in self.end_dangling_comments() {
            for comment in comments {
                builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
            }
        }
    }
}

impl AppendSemanticTokens for ast::RootItem {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        match self {
            Self::Table(table) => table.append_semantic_tokens(builder),
            Self::ArrayOfTables(array) => array.append_semantic_tokens(builder),
            Self::KeyValue(key_value) => key_value.append_semantic_tokens(builder),
        }
    }
}

impl AppendSemanticTokens for ast::Table {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for comment in self.header_leading_comments() {
            builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
        }

        if let Some(token) = self.bracket_start() {
            builder.add_token(TokenType::OPERATOR, token.into())
        }

        if let Some(header) = self.header() {
            for key in header.keys() {
                builder.add_token(TokenType::STRUCT, key.syntax().clone().into());
            }
        }

        if let Some(token) = self.bracket_end() {
            builder.add_token(TokenType::OPERATOR, token.into())
        }

        if let Some(comment) = self.header_tailing_comment() {
            builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into())
        }

        for key_value in self.key_values() {
            key_value.append_semantic_tokens(builder);
        }
    }
}

impl AppendSemanticTokens for ast::ArrayOfTables {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for comment in self.header_leading_comments() {
            builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
        }

        if let Some(token) = self.double_bracket_start() {
            builder.add_token(TokenType::OPERATOR, token.into());
        }

        if let Some(header) = self.header() {
            for key in header.keys() {
                builder.add_token(TokenType::STRUCT, key.syntax().clone().into());
            }
        }

        if let Some(token) = self.double_bracket_end() {
            builder.add_token(TokenType::OPERATOR, token.into());
        }

        if let Some(comment) = self.header_tailing_comment() {
            builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into())
        }

        for key_value in self.key_values() {
            key_value.append_semantic_tokens(builder);
        }
    }
}

impl AppendSemanticTokens for ast::KeyValue {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for comment in self.leading_comments() {
            builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
        }

        if let Some(key) = self.keys() {
            key.append_semantic_tokens(builder)
        }

        if let Some(value) = self.value() {
            value.append_semantic_tokens(builder)
        }
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
        builder.add_token(TokenType::VARIABLE, self.syntax().clone().into());
    }
}

impl AppendSemanticTokens for ast::Value {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for comment in self.leading_comments() {
            builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
        }

        match self {
            Self::BasicString(n) => {
                builder.add_token(TokenType::STRING, (n.token().unwrap()).into())
            }
            Self::LiteralString(n) => {
                builder.add_token(TokenType::STRING, (n.token().unwrap()).into())
            }
            Self::MultiLineBasicString(n) => {
                builder.add_token(TokenType::STRING, (n.token().unwrap()).into())
            }
            Self::MultiLineLiteralString(n) => {
                builder.add_token(TokenType::STRING, (n.token().unwrap()).into())
            }
            Self::IntegerBin(n) => {
                builder.add_token(TokenType::NUMBER, (n.token().unwrap()).into())
            }
            Self::IntegerOct(n) => {
                builder.add_token(TokenType::NUMBER, (n.token().unwrap()).into())
            }
            Self::IntegerDec(n) => {
                builder.add_token(TokenType::NUMBER, (n.token().unwrap()).into())
            }
            Self::IntegerHex(n) => {
                builder.add_token(TokenType::NUMBER, (n.token().unwrap()).into())
            }
            Self::Float(n) => builder.add_token(TokenType::NUMBER, (n.token().unwrap()).into()),
            Self::Boolean(n) => builder.add_token(TokenType::BOOLEAN, (n.token().unwrap()).into()),
            Self::OffsetDateTime(n) => {
                builder.add_token(TokenType::DATETIME, (n.token().unwrap()).into())
            }
            Self::LocalDateTime(n) => {
                builder.add_token(TokenType::DATETIME, (n.token().unwrap()).into())
            }
            Self::LocalDate(n) => {
                builder.add_token(TokenType::DATETIME, (n.token().unwrap()).into())
            }
            Self::LocalTime(n) => {
                builder.add_token(TokenType::DATETIME, (n.token().unwrap()).into())
            }
            Self::Array(array) => array.append_semantic_tokens(builder),
            Self::InlineTable(inline_table) => inline_table.append_semantic_tokens(builder),
        }

        if let Some(comment) = self.tailing_comment() {
            builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into())
        }
    }
}

impl AppendSemanticTokens for ast::Array {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for comments in self.inner_begin_dangling_comments() {
            for comment in comments {
                builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
            }
        }

        for (value, comma) in self.values_with_comma() {
            value.append_semantic_tokens(builder);
            if let Some(comma) = comma {
                for comment in comma.leading_comments() {
                    builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
                }

                if let Some(comment) = comma.tailing_comment() {
                    builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into())
                }
            }
        }

        for comments in self.inner_end_dangling_comments() {
            for comment in comments {
                builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
            }
        }
    }
}

impl AppendSemanticTokens for ast::InlineTable {
    fn append_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        for comments in self.inner_begin_dangling_comments() {
            for comment in comments {
                builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
            }
        }

        for (key_value, comma) in self.key_values_with_comma() {
            key_value.append_semantic_tokens(builder);
            if let Some(comma) = comma {
                for comment in comma.leading_comments() {
                    builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
                }

                if let Some(comment) = comma.tailing_comment() {
                    builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into())
                }
            }
        }

        for comments in self.inner_end_dangling_comments() {
            for comment in comments {
                builder.add_token(TokenType::COMMENT, comment.as_ref().syntax().clone().into());
            }
        }
    }
}
