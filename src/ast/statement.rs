use super::super::types::{Expr, Key, Span, Spanned, Tkn, Value};
use super::expression::expression;
use chumsky::{input::ValueInput, prelude::*};
use ordered_float::OrderedFloat as Float;

pub fn statement<'a, I>()
-> impl Parser<'a, I, Vec<Box<Spanned<Expr<'a>>>>, extra::Err<Rich<'a, Tkn<'a>, Span>>> + Clone
where
    I: ValueInput<'a, Token = Tkn<'a>, Span = Span>,
{
    let kw = |c: &'a str| just(Tkn::Keyword(c)).labelled("Keyword");
    let sym = |c: &'a str| just(Tkn::Symbol(c)).labelled("Symbol");
    let del = |c: char| just(Tkn::Delimiter(c));

    let ident = select! { Tkn::Identifier(ident) => ident }.labelled("identifier");

    let stmt_term = sym(";")
        .labelled("';'")
        .or(just(Tkn::Newline).labelled("'\\n'"));

    let nl = just(Tkn::Newline).or_not();

    let complex_term = recursive(|complex_term| {
        let complex = recursive(|complex| {
            let block_recovery = nested_delimiters(
                Tkn::Delimiter('{'),
                Tkn::Delimiter('}'),
                [
                    (Tkn::Delimiter('('), Tkn::Delimiter(')')),
                    (Tkn::Delimiter('['), Tkn::Delimiter(']')),
                ],
                |span| (Expr::Error, span),
            );

            let block = complex_term
                .clone()
                .repeated()
                .collect::<Vec<Box<_>>>()
                .delimited_by(del('{'), del('}'))
                .map_with(|v, e| (Expr::Block(v), e.span()))
                .recover_with(via_parser(block_recovery));

            let primitive = select! {
            Tkn::Number(f) => Value::Num(f),
            Tkn::Str(f) => Value::Str(f)
            };

            let object = primitive
                .clone()
                .map(|v| match v {
                    Value::Num(a) => Key::Num(Float(a)),
                    Value::Str(a) => Key::Str(a),
                    _ => Key::Str("".to_string()),
                })
                .padded_by(nl.clone())
                .then_ignore(sym(":"))
                .padded_by(nl.clone())
                .then(complex.clone())
                .padded_by(nl.clone())
                .separated_by(sym(",").padded_by(nl.clone()))
                .allow_trailing()
                .collect()
                .delimited_by(del('{').padded_by(nl.clone()), del('}'))
                .map(Value::Obj)
                .map(Expr::Val)
                .map_with(|expr, e| Box::new((expr, e.span())));

            let expression = expression(object.clone());

            let r#let = kw("let")
                .ignore_then(ident)
                .then(choice((
                    (any().and_is(sym("=").not()).rewind())
                        .map_with(|_, e| Box::new((Expr::Val(Value::Undefined), e.span()))),
                    sym("=").ignore_then(expression.clone()),
                )))
                .map(|(name, rhs)| Expr::Let(name, rhs));

            let assign = ident
                .then_ignore(sym("="))
                .then(expression.clone())
                .map(|(name, rhs)| Expr::Assign(name, rhs));

            let r#fn = kw("fn")
                .ignore_then(ident)
                .then(
                    ident
                        .separated_by(sym(","))
                        .allow_trailing()
                        .collect::<Vec<_>>()
                        .delimited_by(del('('), del(')')),
                )
                .then(choice((
                    sym("=").ignore_then(
                        expression
                            .clone()
                            .map_with(|s, e| (Expr::Return(s), e.span())),
                    ),
                    block.clone(),
                )))
                .map(|((name, args), body)| Expr::Fn(name, args, Box::new(body)));

            let r#return = kw("return")
                .ignore_then(expression.clone())
                .map(Expr::Return);

            let r#if = kw("if")
                .ignore_then(expression.clone())
                .then_ignore(stmt_term.clone().or_not())
                .then(choice((complex.clone(), block.clone().map(Box::new))))
                .then(
                    (
                        kw("else").ignore_then(
                            choice((complex.clone(), block.clone().map(Box::new))).or_not(),
                        )
                        //.then_ignore(stmt_term.clone().or_not())
                    )
                    .or_not(),
                )
                .map_with(|((condition, r#if), r#else), e| {
                    (
                        Expr::If(
                            condition,
                            r#if,
                            (r#else.unwrap_or_else(|| {
                                Some(Box::new((Expr::Val(Value::Undefined), e.span())))
                            }))
                            .unwrap_or_else(|| Box::new((Expr::Val(Value::Undefined), e.span()))),
                        ),
                        e.span(),
                    )
                });

            let r#while = kw("while")
                .ignore_then(expression.clone())
                .then(choice((complex.clone(), block.clone().map(Box::new))))
                .map(|(condition, body)| Expr::While(condition, body));

            assign
                .or(r#let)
                .or(r#fn)
                .or(r#return)
                .or(r#while)
                .map_with(|expr, e| (expr, e.span()))
                .or(r#if)
                .map(Box::new)
                .or(expression.clone())
        });

        complex
            .then_ignore(
                stmt_term
                    .clone()
                    .to(())
                    .or(end().rewind().labelled("EOF").to(())),
            )
            .padded_by(stmt_term.repeated().or_not())
    });

    complex_term.repeated().collect::<Vec<_>>()
}
