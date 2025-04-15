use chumsky::prelude::*;

use super::types::{Span, Spanned, Tkn};

pub fn lex<'a>() -> impl Parser<'a, &'a str, Vec<Spanned<Tkn<'a>>>, extra::Err<Rich<'a, char, Span>>>
{
    let newline = text::newline().map(|_| Tkn::Newline);

    let keyword = choice((
        just("let"),
        just("fn"),
        just("return"),
        just("if"),
        just("else"),
        just("while"),
    ))
    .map(Tkn::Keyword);

    let number = text::int(10).map(|s: &str| Tkn::Number(s.parse().unwrap()));

    let identifier = text::ascii::ident()
        .and_is(keyword.not())
        .map(Tkn::Identifier);

    let math_sym = choice((
        just("++"),
        just("--"),
        just("/"),
        just("'"),
        just("+"),
        just("-"),
        just("*"),
    ))
    .map(Tkn::Symbol);

    let logic_sym = choice((
        just("<="),
        just("<="),
        just("=="),
        just("!="),
        just("/*"),
        just("*/"),
        just("//"),
        just("\\"),
        just("\""),
        just("'"),
        just("&"),
        just("|"),
        just("!"),
        just(">"),
        just("<"),
        just("="),
        just(","),
        just("."),
        just(":"),
        just(";"),
        just("#"),
    ))
    .map(Tkn::Symbol);

    let delimiter = choice((
        just('('),
        just(')'),
        just('['),
        just(']'),
        just('{'),
        just('}'),
    ))
    .map(Tkn::Delimiter);

    let comment = choice((
        just("/*")
            .then(any().and_is(just("*/").not()).repeated())
            .then_ignore(just("*/")),
        just("//").then(any().and_is(just('\n').not()).repeated()),
    ));

    newline
        .or(number)
        .or(keyword)
        .or(logic_sym)
        .or(math_sym)
        .or(delimiter)
        .or(identifier)
        .padded_by(text::inline_whitespace())
        .padded_by(comment.repeated())
        .map_with(|tok, e| (tok, e.span()))
        .repeated()
        .collect()
        .boxed()
}
