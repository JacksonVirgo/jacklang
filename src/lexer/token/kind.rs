pub enum TokenKind {
    Unknown(String),
    EndOfFile,

    Integer(i64),
    Identifier(String),

    Assign,
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
