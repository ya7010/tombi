use ast::{AstNode, AstToken};

use super::{AppendSemanticTokens, SemanticTokensBuilder, TokenType};

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
