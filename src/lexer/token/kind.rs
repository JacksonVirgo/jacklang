#[derive(Debug)]
pub enum TokenKind {
    Unknown(String),
    EndOfFile,
    EndOfLine,

    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),

    // Grouping
    ParenLeft,
    ParenRight,
    AngleLeft,
    AngleRight,

    // Operators
    Plus,
    Minus,
    Asterix,
    SlashFwd,

    Equal,
    CompareEqual,
    CompareGreaterEqual,
    CompareLesserEqual,

    // Reserved Keywords
    Let,
}

impl TokenKind {
    pub fn from_ident(ident: String) -> Self {
        use TokenKind::*;
        match ident.as_str() {
            "let" => Let,
            _ => TokenKind::Identifier(ident),
        }
    }
}
