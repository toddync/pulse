#[allow(warnings)]
#[derive(Debug)]
pub enum Expr<'a> {
    Comment,
    Undefined,
    Num(f64),
    Var(&'a str),

    Neg(Box<Expr<'a>>),
    Not(Box<Expr<'a>>),

    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),

    Eq(Box<Expr<'a>>, Box<Expr<'a>>),
    Neq(Box<Expr<'a>>, Box<Expr<'a>>),

    Gt(Box<Expr<'a>>, Box<Expr<'a>>),
    Gte(Box<Expr<'a>>, Box<Expr<'a>>),

    Lt(Box<Expr<'a>>, Box<Expr<'a>>),
    Lte(Box<Expr<'a>>, Box<Expr<'a>>),

    Let {
        name: &'a str,
        rhs: Box<Expr<'a>>,
    },
    Assign {
        name: &'a str,
        rhs: Box<Expr<'a>>,
    },

    Fn {
        name: &'a str,
        args: Vec<&'a str>,
        body: Vec<Expr<'a>>,
    },
    Return(Box<Expr<'a>>),
    Call(&'a str, Vec<Expr<'a>>),

    If {
        condition: Box<Expr<'a>>,
        body: Vec<Expr<'a>>,
    },

    IfElse {
        condition: Box<Expr<'a>>,
        r#true: Vec<Expr<'a>>,
        r#false: Vec<Expr<'a>>,
    },

    While {
        condition: Box<Expr<'a>>,
        body: Vec<Expr<'a>>,
    },
}
