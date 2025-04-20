use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::prelude::*;

use std::{
    env, fs,
    time::{SystemTime, UNIX_EPOCH},
};

use ast::{lex::lex, parse};
mod ast;
pub mod types;

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let source = fs::read_to_string(&filename).unwrap();
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let (tokens, lex_errs) = lex().parse(&source).into_output_errors();
    if tokens.is_none() {
        show_errors(lex_errs, filename, &source);
        return;
    }

    let tokens = tokens.unwrap();
    let (ast, parse_errs) = parse(&tokens, &source);

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start_time;

    show_errors(parse_errs, filename, &source);
    println!("{:#?}", ast);
    println!("time: {:?}", time);
}

fn show_errors<'a, T>(errs: Vec<Rich<'a, T>>, filename: String, src: &String)
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
