use crate::{
    errors::builder::{ContextPosition, ScriptAlert},
    lexer::Lexer,
};

impl Lexer {
    pub fn error(&mut self) -> ScriptAlert {
        let (start_line, start_col) = self.line_of(Some(self.token_start));
        let (_, end_col) = self.line_of(Some(self.cursor_pos));
        let mut alert = ScriptAlert::new();
        alert
            .with_line(start_line)
            .with_context(self.get_line_data(self.cursor_pos))
            .with_position(Some(if self.cursor_pos == self.token_start {
                ContextPosition::Point(start_col)
            } else {
                ContextPosition::Range((start_col, end_col))
            }));
        alert
    }
}
