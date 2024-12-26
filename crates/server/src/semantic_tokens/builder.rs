use tower_lsp::lsp_types::{Position, Range, SemanticToken};

use super::token_type::TokenType;

pub struct SemanticTokensBuilder<'a> {
    tokens: Vec<SemanticToken>,
    last_range: Range,
    source: &'a str,
}

impl<'a> SemanticTokensBuilder<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            last_range: Range::default(),
            source,
        }
    }

    pub fn add_token(&mut self, token_type: TokenType, elem: syntax::SyntaxElement) {
        let range = Range::new(
            text::Position::from_source(self.source, elem.span().start()).into(),
            text::Position::from_source(self.source, elem.span().end()).into(),
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

    pub fn build(self) -> Vec<SemanticToken> {
        self.tokens
    }
}

fn relative_position(position: Position, to: Position) -> Position {
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

fn relative_range(from: Range, to: Range) -> Range {
    let line_diff = from.end.line - from.start.line;
    let start = relative_position(from.start, to.start);

    let end = if line_diff == 0 {
        Position {
            line: start.line,
            character: start.character + from.end.character - from.start.character,
        }
    } else {
        Position {
            line: start.line + line_diff,
            character: from.end.character,
        }
    };

    Range { start, end }
}
