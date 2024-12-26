use tower_lsp::lsp_types::{Position, Range, SemanticToken};

use super::token_type::TokenType;

pub struct SemanticTokensBuilder {
    tokens: Vec<SemanticToken>,
    last_range: text::Range,
}

impl SemanticTokensBuilder {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            last_range: text::Range::default(),
        }
    }

    pub fn add_token(&mut self, token_type: TokenType, elem: syntax::SyntaxElement) {
        let range = elem.range();

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

    pub fn build(self) -> Vec<SemanticToken> {
        self.tokens
    }
}

fn relative_range(from: text::Range, to: text::Range) -> Range {
    let line_diff = from.end().line() - from.start().line();
    let start = from.start() - to.start();
    let start = Position {
        line: start.line(),
        character: start.column(),
    };

    let end = if line_diff == 0 {
        Position {
            line: start.line,
            character: start.character + from.end().column() - from.start().column(),
        }
    } else {
        Position {
            line: start.line + line_diff,
            character: from.end().column(),
        }
    };

    Range { start, end }
}
