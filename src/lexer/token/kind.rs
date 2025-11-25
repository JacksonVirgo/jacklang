pub enum TokenKind {
    Unknown(String),
    EndOfFile,

    // Literals
    Integer(i64),
    Float(f64),
    String(String),

    Identifier(String),

    Equal,
    CompareEqual,

    // Grouping
    ParenLeft,
    ParenRight,

    // Operators
    Plus,
    Minus,
    Asterix,
    SlashFwd,

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
