use super::types::{Expr, Spanned, Tkn};
use statement::statement;
use chumsky::prelude::*;

pub mod lex;
mod statement;
mod expression;

pub fn parse<'a>(tkn: &'a [Spanned<Tkn<'a>>], source: &str) -> (Option<(Vec<Box<(Expr<'a>, SimpleSpan)>>, SimpleSpan)>, Vec<chumsky::error::Rich<'a, Tkn<'a>>>) {
    statement()
        .map_with(|ast, e| (ast, e.span()))
        .parse(
            tkn
                .map((source.len()..source.len()).into(), |(t, s)| (t, s)),
        )
        .into_output_errors()
}
