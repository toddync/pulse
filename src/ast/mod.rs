use super::types::{Expr, Spanned, Tkn};
use ariadne::{Color, Label, Report, ReportKind, sources};
use chumsky::prelude::*;
use statement::statement;

mod expression;
mod lex;
mod statement;

pub fn lex<'a>(
    source: &'a str,
) -> (
    Option<Vec<Spanned<Tkn<'a>>>>,
    Vec<chumsky::error::Rich<'a, char>>,
) {
    lex::lex().parse(&source).into_output_errors()
}

pub fn parse<'a>(
    tkn: &'a [Spanned<Tkn<'a>>],
    source: &str,
) -> (
    Option<Vec<Box<(Expr<'a>, SimpleSpan)>>>,
    Vec<chumsky::error::Rich<'a, Tkn<'a>>>,
) {
    statement()
        .parse(tkn.map((source.len()..source.len()).into(), |(t, s)| (t, s)))
        .into_output_errors()
}

pub fn show_errors<'a, T>(errs: Vec<Rich<'a, T>>, filename: String, src: &String)
where
    T: ToString + Clone,
{
    errs.into_iter()
        .map(|e| e.map_token(|c| c.to_string()))
        .for_each(|e| {
            Report::build(ReportKind::Error, (filename.clone(), e.span().into_range()))
                .with_message(e.to_string())
                .with_label(
                    Label::new((filename.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((filename.clone(), span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .print(sources([(filename.clone(), src.clone())]))
                .unwrap()
        })
}
