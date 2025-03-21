use chumsky::prelude::*;

#[derive(Debug)]
enum Expr<'a> {
    Num(f64),
    Var(&'a str),

    Neg(Box<Expr<'a>>),
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),

    Call(&'a str, Vec<Expr<'a>>),
    Let { name: &'a str, rhs: Box<Expr<'a>> },
    Fn { name: &'a str, args: Vec<&'a str>, body: Vec<Expr<'a>> },
}

#[allow(clippy::let_and_return)]
fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Expr<'a>>> {
    let ident = text::ascii::ident().padded();

    let expr = recursive(|expr| {
        let int = text::int(10).map(|s: &str| Expr::Num(s.parse().unwrap()));

        let call = ident
            .then(
                expr.clone()
                    .separated_by(just(','))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| Expr::Call(f, args));

        let atom = int
            .or(expr.delimited_by(just('('), just(')')))
            .or(call)
            .or(ident.map(Expr::Var))
            .padded();

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary.clone().foldl(
            choice((
                op('*').to(Expr::Mul as fn(_, _) -> _),
                op('/').to(Expr::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let sum = product.clone().foldl(
            choice((
                op('+').to(Expr::Add as fn(_, _) -> _),
                op('-').to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        sum
    });

    let decl = recursive(|decl| {
        let r#let = text::ascii::keyword("let")
            .ignore_then(ident)
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';').or_not())
            .map(|(name, rhs)| Expr::Let {
                name,
                rhs: Box::new(rhs),
            });

        let r#fn = text::ascii::keyword("fn")
            .ignore_then(ident.padded())
            .then(
                ident
                    .separated_by(just(','))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just('('), just(')')),
            )
            .padded()
            .then(
                decl.repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just('{'), just('}')),
            )
            .then_ignore(just(';').or_not())
            .map(|((name, args), body)| Expr::Fn {
                name,
                args,
                body,
            });

        r#let.then_ignore(just(';').or_not())
            .or(r#fn.then_ignore(just(';').or_not()))
            .or(expr.clone().then_ignore(just(';').or_not()))
            .padded()
    });

    decl.repeated().collect()
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("{:#?}",  parser().parse(&src).unwrap())
}