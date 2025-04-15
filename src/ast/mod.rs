use chumsky::{input::ValueInput, prelude::*};
use inline::inline;
use types::{Expr, Span, Spanned, Tkn, Value};

mod inline;
pub mod lex;

pub mod types;

pub fn parser<'a, I>()
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

    let inline = inline();

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

            let r#let = kw("let")
                .ignore_then(ident)
                .then(choice((
                    (any().and_is(sym("=").not()).rewind())
                        .map_with(|_, e| Box::new((Expr::Val(Value::Undefined), e.span()))),
                    sym("=").ignore_then(inline.clone()),
                )))
                .map(|(name, rhs)| Expr::Let(name, rhs));

            let assign = ident
                .then_ignore(sym("="))
                .then(inline.clone())
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
                    sym("=")
                        .ignore_then(inline.clone().map_with(|s, e| (Expr::Return(s), e.span()))),
                    block.clone(),
                )))
                .map(|((name, args), body)| Expr::Fn(name, args, Box::new(body)));

            let r#return = kw("return").ignore_then(inline.clone()).map(Expr::Return);

            let r#if = kw("if")
                .ignore_then(inline.clone())
                .then_ignore(stmt_term.clone().or_not())
                .then(choice((complex.clone(), block.clone())))
                .then(
                    (kw("else")
                        .ignore_then(choice((complex.clone(), block.clone())).or_not())
                        .then_ignore(stmt_term.clone().or_not()))
                    .or_not(),
                )
                .map_with(|((condition, r#if), r#else), e| {
                    (
                        Expr::If(
                            condition,
                            Box::new(r#if),
                            Box::new(
                                (r#else.unwrap_or_else(|| {
                                    Some((Expr::Val(Value::Undefined), e.span()))
                                }))
                                .unwrap_or_else(|| (Expr::Val(Value::Undefined), e.span())),
                            ),
                        ),
                        e.span(),
                    )
                });

            let r#while = kw("while")
                .ignore_then(inline.clone())
                .then(choice((complex.clone(), block.clone())))
                .map(|(condition, body)| Expr::While(condition, Box::new(body)));

            assign
                .or(r#let)
                .or(r#fn)
                .or(r#return)
                .or(r#while)
                .map_with(|expr, e| (expr, e.span()))
                .or(r#if)
                .or(inline.map(|e| *e))
        });

        
        complex
            .then_ignore(
                stmt_term
                    .clone()
                    .to(())
                    .or(end().rewind().labelled("EOF").to(())),
            )
        .padded_by(stmt_term.repeated().or_not())
        .map(Box::new)
    });

    complex_term.repeated().collect::<Vec<_>>()
}
