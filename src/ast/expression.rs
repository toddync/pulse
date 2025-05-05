use super::super::types::{Expr, Op, Span, Spanned, Tkn, Value};
use chumsky::{input::ValueInput, prelude::*};

pub fn expression<'a, I>(
    ext_val: impl Parser<'a, I, Box<Spanned<Expr<'a>>>, extra::Err<Rich<'a, Tkn<'a>, Span>>>
    + Clone
    + 'a,
) -> impl Parser<'a, I, Box<Spanned<Expr<'a>>>, extra::Err<Rich<'a, Tkn<'a>, Span>>> + Clone
where
    I: ValueInput<'a, Token = Tkn<'a>, Span = Span>,
{
    let sym = |c: &'a str| just(Tkn::Symbol(c));

    let del = |c: char| just(Tkn::Delimiter(c));
    let del_ = |c: char| Tkn::Delimiter(c);

    let ident = select! { Tkn::Identifier(ident) => ident }.labelled("identifier");
    //let ident_ = |c: &str| just(Tkn::Identifier(c));

    let nl = just(Tkn::Newline).or_not();

    recursive(|inline| {
        let primitive = select! {
            Tkn::Number(f) => Value::Num(f),
            Tkn::Bool(f) => Value::Bool(f),
            Tkn::Str(f) => Value::Str(f)
        }
        .labelled("Number");

        let items = inline
            .clone()
            .padded_by(nl.clone())
            .separated_by(sym(",").padded_by(nl.clone()))
            .collect::<Vec<Box<_>>>();

        let vec = items
            .clone()
            .delimited_by(del('['), del(']'))
            .map(Value::Vec);

        let val = choice((vec, primitive)).map(Expr::Val);

        let atom = val
            .or(ident.map(Expr::Var))
            .map_with(|expr, e| Box::new((expr, e.span())))
            .or(ext_val.clone())
            .or(inline.clone().delimited_by(del('('), del(')')));

        let call = atom
            .clone()
            .then(items.clone().delimited_by(del('('), del(')')))
            .map(|(f, args)| Expr::Call(f, args))
            .map_with(|expr, e| Box::new((expr, e.span())));

        let reserved_fn = select! { Tkn::Identifier("print") => "print" }
            .then(items.clone().delimited_by(del('('), del(')')))
            .map(|(f, args)| Expr::ReservedCall(f, args))
            .map_with(|expr, e| Box::new((expr, e.span())));

        let at = choice((reserved_fn, call, atom));

        let op = sym("!").to(Op::Not).or(sym("-").to(Op::Neg));
        let unary = op
            .repeated()
            .foldr_with(at.clone(), |op, rhs, e| {
                Box::new((Expr::UnOp(op, rhs), e.span()))
            })
            .recover_with(via_parser(
                any()
                    .filter(|tok| {
                        !matches!(
                            tok,
                            Tkn::Keyword(_)
                                | Tkn::Delimiter(_)
                                | Tkn::Identifier(_)
                                | Tkn::Number(_)
                                | Tkn::Symbol(";")
                        )
                    })
                    .map_with(|_, e| Box::new((Expr::Error, e.span()))),
            ));

        let op = sym("*").to(Op::Mul).or(sym("/").to(Op::Div));
        let product = unary
            .clone()
            .foldl_with(op.then(unary.clone()).repeated(), |a, (op, b), e| {
                Box::new((Expr::BnOp(a, op, b), e.span()))
            });

        let op = sym("+").to(Op::Add).or(sym("-").to(Op::Sub));
        let sum = product
            .clone()
            .foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                Box::new((Expr::BnOp(a, op, b), e.span()))
            });

        let op = choice((
            sym("&").then_ignore(sym("&")).to(Op::And),
            sym("|").then_ignore(sym("|")).to(Op::Or),
            sym("%").to(Op::Mod),
            sym("==").to(Op::Eq),
            sym("!=").to(Op::Neq),
            sym(">=").to(Op::Gte),
            sym("<=").to(Op::Lte),
            sym(">").to(Op::Gt),
            sym("<").to(Op::Lt),
        ))
        /*.recover_with(via_parser(
            any()
                .and_is(just(Tkn::Symbol(";")).not())
                .and_is(just(Tkn::Symbol(",")).not())
                .filter(|tok| {
                    !matches!(
                        tok,
                        Tkn::Keyword(_) | Tkn::Delimiter(_) | Tkn::Identifier(_)
                    )
                })
                .map(|_| Op::Error),
        )) */;

        let compare = choice((
            sum.clone()
                .foldl_with(op.then(sum).repeated(), |a, (op, b), e| {
                    Box::new((Expr::BnOp(a, op, b), e.span()))
                }),
            at.clone(),
        ))
        .recover_with(via_parser(nested_delimiters(
            del_('('),
            del_(')'),
            [(del_('['), del_(']')), (del_('{'), del_('}'))],
            |span| Box::new((Expr::Error, span)),
        )))
        .recover_with(via_parser(nested_delimiters(
            del_('['),
            del_(']'),
            [(del_('('), del_(')')), (del_('{'), del_('}'))],
            |span| Box::new((Expr::Error, span)),
        )))
        .recover_with(via_parser(
            any()
                .filter(|tok| !matches!(tok, Tkn::Keyword(_) | Tkn::Delimiter(_)))
                .map_with(|_, e| Box::new((Expr::Error, e.span()))),
        ));

        compare.labelled("expression")
    })
}
