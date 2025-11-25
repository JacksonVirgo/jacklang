use crate::lexer::{Lexer, token::kind::TokenKind};

impl Lexer {
    pub fn string(&mut self) -> Option<TokenKind> {
        while self.peek_ahead(1) != Some('"') && !self.is_at_end() {
            if self.peek_ahead(1) == Some('\n') {
                return None;
            }
            self.advance();
        }

        if self.is_at_end() {
            return None;
        }

        self.advance();

        let start = self.token_line;
        let current = self.cursor_pos;

        let value: String = self.source[start + 1..current - 1].iter().collect();
        Some(TokenKind::String(value))
    }

    pub fn number(&mut self) -> TokenKind {
        while self.peek_ahead(1).is_some_and(|d| d.is_digit(10)) {
            self.advance();
        }

        let mut is_float = false;
        if self.peek_ahead(1).is_some_and(|d| d == '.')
            && self.peek_ahead(2).is_some_and(|d| d.is_digit(10))
        {
            is_float = true;
            self.advance();
            while self.peek_ahead(1).is_some_and(|d| d.is_digit(10)) {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.token_start..self.cursor_pos]
            .iter()
            .collect();

        if is_float {
            match lexeme.parse::<f64>() {
                Ok(val) => TokenKind::Float(val),
                Err(e) => {
                    let msg = if e.to_string().contains("out of range") {
                        "Float out of bounds".into()
                    } else {
                        format!("Malformed float literal: '{}'", lexeme)
                    };
                    TokenKind::Unknown(msg)
                }
            }
        } else {
            match lexeme.parse::<i64>() {
                Ok(val) => TokenKind::Integer(val),
                Err(e) => {
                    let msg = match e.kind() {
                        std::num::IntErrorKind::PosOverflow => {
                            "Integer larger than the max permitted value.".into()
                        }
                        std::num::IntErrorKind::NegOverflow => {
                            "Integer smaller than the min permitted value.".into()
                        }
                        _ => {
                            format!("Malformed number: '{}'", lexeme)
                        }
                    };
                    TokenKind::Unknown(msg)
                }
            }
        }
    }

    pub fn identifier(&mut self) -> TokenKind {
        while self.peek_ahead(1).is_some_and(|p| p.is_alphanumeric()) {
            self.advance();
        }
        let lexeme: String = self.source[self.token_start..self.cursor_pos]
            .iter()
            .collect();
        TokenKind::from_ident(lexeme)
    }
}
