use crate::lexer::token::{Token, kind::TokenKind, position::TokenPosition};

pub mod errors;
pub mod multi_char;
pub mod stream;
pub mod token;

pub struct Lexer {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,

    pub token_start: usize,
    pub token_line: usize,
    pub cursor_pos: usize,

    pub current_lexeme: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],

            token_start: 0,
            token_line: 1,
            cursor_pos: 0,

            current_lexeme: String::new(),
        }
    }

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.token_start = self.cursor_pos;
            self.current_lexeme = String::new();
            self.parse_token();
        }

        self.tokens.push(Token::new(
            TokenKind::EndOfFile,
            (self.token_start, self.token_line),
            "",
        ));
    }

    pub fn parse_token(&mut self) {
        let Some(c) = self.advance() else {
            return;
        };

        if c.is_whitespace() {
            return;
        }

        let mut token = Token::default();
        token.position = TokenPosition::from((self.token_line, self.token_start));

        let kind = match c {
            '(' => TokenKind::ParenLeft,
            ')' => TokenKind::ParenRight,
            '=' => {
                if self.peek_ahead(1).is_some_and(|c| c == '=') {
                    TokenKind::CompareEqual
                } else {
                    TokenKind::Equal
                }
            }
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterix,
            '/' => TokenKind::SlashFwd,
            '"' => match self.string() {
                Some(s) => s,
                None => TokenKind::Unknown("Unterminated string".into()),
            },
            _ => {
                if c.is_alphabetic() {
                    self.identifier()
                } else if c.is_numeric() {
                    self.number()
                } else {
                    TokenKind::Unknown("Unexpected token found".into())
                }
            }
        };

        if let TokenKind::Unknown(msg) = &kind {
            self.error().with_message(msg).report();
            return;
        }
        token.lexeme = self.current_lexeme.clone();
        token.kind = kind;
    }
}
