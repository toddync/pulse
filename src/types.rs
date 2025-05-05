#![allow(warnings)]
use chumsky::span::SimpleSpan;
use core::fmt;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);
pub type BSE<'a> = Box<Spanned<Expr<'a>>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Str(String),
    Num(OrderedFloat<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    Undefined,
    Num(f64),
    Str(String),
    Bool(bool),
    Vec(Vec<BSE<'a>>),
    Obj(HashMap<Key, BSE<'a>>),
    Fn(Vec<&'a str>, BSE<'a>),
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
    Mod,

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
    ReservedVar(&'a str),

    Val(Value<'a>),

    UnOp(Op, BSE<'a>),
    BnOp(BSE<'a>, Op, BSE<'a>),

    Let(&'a str, BSE<'a>),
    Assign(&'a str, BSE<'a>),

    Block(Vec<BSE<'a>>),

    Fn(&'a str, Vec<&'a str>, BSE<'a>),
    Return(BSE<'a>),
    Call(BSE<'a>, Vec<BSE<'a>>),

    ReservedCall(&'a str, Vec<BSE<'a>>),

    If(BSE<'a>, BSE<'a>, BSE<'a>),

    While(BSE<'a>, BSE<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tkn<'a> {
    Number(f64),
    Str(String),
    Bool(bool),

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
            Tkn::Bool(n) => write!(f, "{}", n),
            Tkn::Str(n) => write!(f, "{}", n),
            Tkn::Identifier(c) => write!(f, "{}", c),
            Tkn::Delimiter(c) => write!(f, "{}", c),
            Tkn::Keyword(c) => write!(f, "{}", c),
            Tkn::Symbol(c) => write!(f, "{}", c),
            Tkn::Newline => write!(f, "\\n"),
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Num(x) => write!(f, "{}", x),
            Value::Str(x) => write!(f, "{}", x),
            Value::Bool(x) => write!(f, "{}", x),
            Value::Vec(_) => write!(f, ""),
            Value::Obj(_) => write!(f, ""),
            Value::Undefined => write!(f, "undefined"),
            Value::Fn(_, _) => write!(f, "(Function () => {{}})"),
        }
    }
}
