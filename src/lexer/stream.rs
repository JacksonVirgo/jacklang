use crate::lexer::Lexer;

impl Lexer {
    pub fn is_at_end(&mut self) -> bool {
        self.cursor_pos >= self.source.len()
    }

    pub fn get_line_data(&self, cursor_pos: usize) -> String {
        let mut start = cursor_pos;
        while start > 0 && self.source[start - 1] != '\n' {
            start -= 1;
        }

        let mut end = cursor_pos;
        while end < self.source.len() && self.source[end] != '\n' {
            end += 1;
        }

        self.source[start..end].iter().collect()
    }

    pub fn line_of(&self, index: Option<usize>) -> (usize, usize) {
        let idx = index.unwrap_or(self.cursor_pos);
        let line_start = self.source[..idx]
            .iter()
            .rposition(|&c| c == '\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let column = idx - line_start;
        (self.token_line, column)
    }

    pub fn advance(&mut self) -> Option<char> {
        let char = self.source.get(self.cursor_pos);
        self.cursor_pos += 1;

        if let Some(c) = char {
            self.current_lexeme.push(c.clone());
        }

        char.cloned()
    }

    pub fn peek_ahead(&mut self, amount: usize) -> Option<char> {
        if self.cursor_pos + amount - 1 >= self.source.len() {
            return None;
        }
        self.source.get(self.cursor_pos + amount - 1).cloned()
    }
}
