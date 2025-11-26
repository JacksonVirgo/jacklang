use crate::lexer::token::Token;

pub enum Stmt {
    Program(Program),
    Expr(Expr),
    FunctionDecl,
}

pub enum Expr {
    Identifier(String),
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Call,
    Unary,
}

pub struct Program {
    pub body: Box<Expr>,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub enum LiteralExpr {
    Integer(u64),
    Float(f64),
    String(String),
    Boolean(bool),
}
