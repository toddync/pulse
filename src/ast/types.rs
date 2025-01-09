#![allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    String(String),
    Not(Box<Expr>),
    Number(i128),
    Float(f64),
    Bool(bool),
    Empty(),
    Variable(String),

    Range {
        iterator: Box<Expr>,
        start: Box<Expr>,
        end: Box<Expr>,
        step: i128,
    },

    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },

    VarDec {
        name: String,
        value: Box<Expr>,
        line: usize,
    },
    Assign {
        name: String,
        value: Box<Expr>,
        line: usize,
    },

    If {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },

    Else {
        body: Vec<Expr>,
    },

    ElIf {
        condition: Box<Expr>,
        valid: Vec<Expr>,
        invalid: Vec<Expr>,
    },

    FnCall {
        name: String,
        params: Vec<Expr>,
    },

    For {
        domain: Box<Expr>,
        body: Vec<Expr>,
    },

    While {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum RpnExpr {
    Basic(String),
    Complex(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    //* Math
    Subtract,
    Multiply,
    Modulus,
    Divide,
    Power,
    Add,

    //* Logic
    Different,
    LtEquals,
    GtEquals,
    Greater,
    Equals,
    Less,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //* variables, reserved words and chars
    Identifier,
    Semicolon,
    Delimiter,
    Keyword,
    Comma,

    //* literals
    String,
    Number,
    Float,
    Bool,

    //* math
    Operator,

    //* assignment
    OpAssign,
    Assign,

    //* logic
    Equal,
    Not,
    And,
    Or,

    //* Dots, by quantity
    Dot,
    Range,
    Spread,

    //* others
    Eof,
    Nl,
}
