
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Variable(String),
    String(String),
    Not(Box<Expr>),
    Number(i64),
    Float(f64),
    Bool(bool),
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>
    },
    If {
        condition: Box<Expr>,
        body: Vec<Expr>
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    // NotEqual,
    // Greater,
    // GtEqual,
    // Smaller,
    // SmEqual,
    Equal,
    Not,
    And,
    Or,

    //* others
    EOF,
    Nl,
}