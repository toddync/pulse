use chumsky::prelude::*;
use types::Expr;

pub mod types;

#[allow(clippy::let_and_return)]
pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Expr<'a>>> {
    let ident = text::ascii::ident()
        .and_is(text::keyword("fn").not())
        .and_is(text::keyword("let").not())
        .and_is(text::keyword("if").not())
        .and_is(text::keyword("else").not())
        .padded();

    let simple = recursive(|simple| {
        let int = text::int(10).map(|s: &str| Expr::Num(s.parse().unwrap()));

        let call = ident
            .clone()
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
            .or(ident.clone().map(Expr::Var))
            .padded();

        let op = |c: String| just(c).padded();

        let unary = op("-".to_string())
            .repeated()
            .foldr(atom.clone(), |_op, rhs| Expr::Neg(Box::new(rhs)))
            .or(op("!".to_string())
                .repeated()
                .foldr(atom.clone(), |_op, rhs| Expr::Not(Box::new(rhs))));

        let product = unary.clone().foldl(
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

        cmp.padded()
    });

    let complex = recursive(|complex| {
        let block = complex
            .clone()
            .then_ignore(just(';').or_not())
            .repeated()
            .collect::<Vec<_>>()
            .padded()
            .delimited_by(just('{'), just('}'))
            .or_not()
            .then_ignore(just(';').or_not());

        let r#let = text::ascii::keyword("let")
            .ignore_then(ident.clone())
            .then(choice((
                just('=').ignore_then(simple.clone()),
                just(';').map(|_| Expr::Undefined),
            )))
            .then_ignore(just(';').or_not())
            .map(|(name, rhs)| Expr::Let {
                name,
                rhs: Box::new(rhs),
            });

        let assign = ident
            .clone()
            .then_ignore(just('='))
            .then(simple.clone())
            .map(|(name, rhs)| Expr::Assign {
                name,
                rhs: Box::new(rhs),
            });

        let r#fn = text::ascii::keyword("fn")
            .ignore_then(ident.clone().padded())
            .then(
                ident
                    .clone()
                    .separated_by(just(','))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just('('), just(')')),
            )
            .then(choice((
                just('=').padded().ignore_then(
                    simple
                        .clone()
                        .map(|s| Some(vec![Expr::Return(Box::new(s))])),
                ),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .map(|((name, args), body)| Expr::Fn {
                name,
                args,
                body: body.unwrap_or(vec![]),
            });

        let r#if = text::ascii::keyword("if")
            .ignore_then(simple.clone())
            .then(choice((
                complex.clone().map(|s| Some(vec![s])),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .map(|(condition, body)| Expr::If {
                condition: Box::new(condition),
                body: body.unwrap_or_else(Vec::new),
            });

        let ifelse = text::ascii::keyword("if")
            .ignore_then(simple.clone())
            .then(choice((
                complex.clone().map(|s| Some(vec![s])),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .then_ignore((text::ascii::keyword("else")).padded())
            .then(choice((
                complex.clone().map(|s| Some(vec![s])),
                block.clone(),
            )))
            .then_ignore(just(';').or_not())
            .map(|((a, b), c)| Expr::IfElse {
                condition: Box::new(a),
                r#true: b.unwrap_or_else(Vec::new),
                r#false: c.unwrap_or_else(Vec::new),
            });

        let comment = choice((
            just("/*").then(any().and_is(just("*/").not()).repeated()),
            just("//").then(any().and_is(just('\n').not()).repeated()),
        ))
        .ignored()
        .map(|_| Expr::Comment);

        comment
            .or(r#let)
            .or(assign)
            .or(ifelse)
            .or(r#if)
            .or(r#fn)
            .or(simple.then_ignore(just(';').or_not()))
            .padded()
    });

    complex.repeated().collect()
}
