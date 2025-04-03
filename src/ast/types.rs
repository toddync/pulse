#[allow(warnings)]
#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'a> {
    Comment(String),
    Nl,
    Undefined,
    Num(f64),
    Var(&'a str),

    Neg(Box<Expr<'a>>),
    Not(Box<Expr<'a>>),

    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),

    And(Box<Expr<'a>>, Box<Expr<'a>>),
    Or(Box<Expr<'a>>, Box<Expr<'a>>),

    Eq(Box<Expr<'a>>, Box<Expr<'a>>),
    Neq(Box<Expr<'a>>, Box<Expr<'a>>),
    Gt(Box<Expr<'a>>, Box<Expr<'a>>),
    Gte(Box<Expr<'a>>, Box<Expr<'a>>),
    Lt(Box<Expr<'a>>, Box<Expr<'a>>),
    Lte(Box<Expr<'a>>, Box<Expr<'a>>),

    Let(&'a str, Box<Expr<'a>>),
    Assign(&'a str, Box<Expr<'a>>),

    Fn(&'a str, Vec<&'a str>, Vec<Expr<'a>>),
    Return(Box<Expr<'a>>),
    Call(&'a str, Vec<Expr<'a>>),

    If(Box<Expr<'a>>, Vec<Expr<'a>>),
    IfElse(Box<Expr<'a>>, Vec<Expr<'a>>, Vec<Expr<'a>>),

    While(Box<Expr<'a>>, Vec<Expr<'a>>),
}
