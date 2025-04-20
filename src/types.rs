#![allow(warnings)]
use chumsky::span::SimpleSpan;
use core::fmt;
use std::collections::HashMap;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);
pub type BSE<'a> = Box<Spanned<Expr<'a>>>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    Undefined,
    Num(f64),
    Str(String),
    Vec(Vec<BSE<'a>>),
    Obj(HashMap<&'a str, BSE<'a>>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Error,

    Not,
    Neg,

    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,

    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'a> {
    Error,
    Comment(String),
    Nl,
    Var(&'a str),
    Val(Value<'a>),

    UnOp(Op, BSE<'a>),
    BnOp(BSE<'a>, Op, BSE<'a>),

    Let(&'a str, BSE<'a>),
    Assign(&'a str, BSE<'a>),

    Block(Vec<BSE<'a>>),

    Fn(&'a str, Vec<&'a str>, BSE<'a>),
    Return(BSE<'a>),
    Call(BSE<'a>, Vec<BSE<'a>>),

    If(BSE<'a>, BSE<'a>, BSE<'a>),

    While(BSE<'a>, BSE<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tkn<'a> {
    Number(f64),
    Str(String),

    Identifier(&'a str),
    Delimiter(char),
    Keyword(&'a str),
    Symbol(&'a str),
    Newline,
}

impl fmt::Display for Tkn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tkn::Number(n) => write!(f, "{}", n),
            Tkn::Str(n) => write!(f, "{}", n),
            Tkn::Identifier(c) => write!(f, "{}", c),
            Tkn::Delimiter(c) => write!(f, "{}", c),
            Tkn::Keyword(c) => write!(f, "{}", c),
            Tkn::Symbol(c) => write!(f, "{}", c),
            Tkn::Newline => write!(f, "\\n"),
        }
    }
}
