use crate::lexer::token::{kind::TokenKind, position::TokenPosition};

pub mod kind;
pub mod position;

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
    pub position: TokenPosition,
}

impl Token {
    pub fn new(kind: TokenKind, position: (usize, usize), lexeme: impl Into<String>) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            position: TokenPosition::from(position),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self::new(
            TokenKind::Unknown("Token has not been assigned".into()),
            (0, 0),
            "",
        )
    }
}
