use chumsky::prelude::*;
use types::Expr;

pub mod types;

#[allow(clippy::let_and_return)]
pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Expr<'a>>> {
    let ident = text::ascii::ident()
        .and_is(just("let").not())
        .and_is(just("fn").not())
        .and_is(just("return").not())
        .and_is(just("if").not())
        .and_is(just("else").not())
        .and_is(just("while").not())
        .padded();

    let simple = recursive(|simple| {
        let int = text::int(10).map(|s: &str| Expr::Num(s.parse().unwrap()));

        let call = ident
            .then(
                simple
                    .clone()
                    .separated_by(just(','))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| Expr::Call(f, args));

        let atom = int
            .or(simple.delimited_by(just('('), just(')')))
            .or(call)
            .or(ident.map(Expr::Var))
            .padded_by(text::inline_whitespace());

        let op = |c: String| just(c).padded_by(text::inline_whitespace());

        let unary = op("-".to_string())
            .repeated()
            .foldr(atom.clone(), |_op, rhs| Expr::Neg(Box::new(rhs)))
            .or(op("!".to_string())
                .repeated()
                .foldr(atom.clone(), |_op, rhs| Expr::Not(Box::new(rhs))));

        let and_or = unary.clone().foldl(
            choice((
                op("&&".to_string()).to(Expr::And as fn(_, _) -> _),
                op("||".to_string()).to(Expr::Or as fn(_, _) -> _),
            ))
            .then(unary.clone())
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let product = and_or.clone().foldl(
            choice((
                op("*".to_string()).to(Expr::Mul as fn(_, _) -> _),
                op("/".to_string()).to(Expr::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let sum = product.clone().foldl(
            choice((
                op("+".to_string()).to(Expr::Add as fn(_, _) -> _),
                op("-".to_string()).to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let cmp = sum.clone().foldl(
            choice((
                op("==".to_string()).to(Expr::Eq as fn(_, _) -> _),
                op("!=".to_string()).to(Expr::Neq as fn(_, _) -> _),
                op(">".to_string()).to(Expr::Gt as fn(_, _) -> _),
                op(">=".to_string()).to(Expr::Gte as fn(_, _) -> _),
                op("<".to_string()).to(Expr::Lt as fn(_, _) -> _),
                op("<=".to_string()).to(Expr::Lte as fn(_, _) -> _),
            ))
            .then(sum)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        cmp.padded_by(text::inline_whitespace()).boxed()
    });

    let complex = recursive(|complex| {
        let stmt_term = choice((
            text::newline(),
            just(';').to(()),
            chumsky::primitive::end().rewind().to(()),
        ));

        let block = complex
            .clone()
            .then_ignore(just(';').or_not())
            .repeated()
            .collect::<Vec<_>>()
            .padded()
            .delimited_by(just('{'), just('}'))
            .or_not()
            .padded_by(text::inline_whitespace());

        let r#let = text::ascii::keyword("let")
            .ignore_then(ident)
            .then(choice((
                just('=').ignore_then(simple.clone()),
                (any().rewind()).map(|_| Expr::Undefined),
            )))
            .map(|(name, rhs)| Expr::Let(name, Box::new(rhs)));

        let assign = ident
            .then_ignore(just('='))
            .then(simple.clone())
            .map(|(name, rhs)| Expr::Assign(name, Box::new(rhs)));

        let r#fn = text::ascii::keyword("fn")
            .ignore_then(ident)
            .then(
                ident
                    .separated_by(just(','))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just('('), just(')')),
            )
            .padded_by(text::inline_whitespace())
            .then(choice((
                just('=').ignore_then(
                    simple
                        .clone()
                        .map(|s| Some(vec![Expr::Return(Box::new(s))])),
                ),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .map(|((name, args), body)| Expr::Fn(name, args, body.unwrap_or_else(Vec::new)));

        let r#return = text::ascii::keyword("return")
            .padded_by(text::inline_whitespace())
            .ignore_then(simple.clone())
            .map(|val| Expr::Return(Box::new(val)));

        let r#if = text::ascii::keyword("if")
            .ignore_then(simple.clone())
            .then(choice((
                complex.clone().map(|s| Some(vec![s])),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .map(|(condition, body)| Expr::If(Box::new(condition), body.unwrap_or_else(Vec::new)));

        let ifelse = text::ascii::keyword("if")
            .ignore_then(simple.clone())
            .then(choice((
                complex
                    .clone()
                    .map(|s| Some(vec![s]))
                    .then_ignore(just(';').or_not()),
                block.clone(),
            )))
            .then_ignore((text::ascii::keyword("else")).padded())
            .then(choice((
                complex.clone().map(|s| Some(vec![s])),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .map(|((condition, valid), invalid)| {
                Expr::IfElse(
                    Box::new(condition),
                    valid.unwrap_or_else(Vec::new),
                    invalid.unwrap_or_else(Vec::new),
                )
            });

        let r#while = text::ascii::keyword("while")
            .ignore_then(simple.clone())
            .then(choice((
                complex.clone().map(|s| Some(vec![s])),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .map(|(condition, body)| {
                Expr::While(Box::new(condition), body.unwrap_or_else(Vec::new))
            });

        let comment = choice((
            just("/*").ignore_then(
                any()
                    .and_is(just("*/").not())
                    .repeated()
                    .collect()
                    .then_ignore(just("*/")),
            ),
            just("//").ignore_then(any().and_is(just('\n').not()).repeated().collect()),
        ))
        .map(Expr::Comment);

        comment
            .or(r#let)
            .or(assign)
            .or(r#while)
            .or(ifelse)
            .or(r#if)
            .or(r#fn)
            .or(r#return)
            .or(simple)
            .then_ignore(stmt_term)
            .boxed()
    });

    complex.padded().repeated().collect()
}
