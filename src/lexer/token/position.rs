#[derive(Debug, Clone, Copy)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

impl From<(usize, usize)> for TokenPosition {
    fn from((line, column): (usize, usize)) -> Self {
        Self { line, column }
    }
}
