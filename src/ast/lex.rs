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

    let digits = text::digits(10).to_slice();
            
    let frac = just('.').then(digits);

    let number = just('-')
            .or_not()
            .then(text::int(10))
            .then(frac.or_not())
            .to_slice()
            .map(|s: &str| s.parse().unwrap())
            .map(Tkn::Number);

        /* let escape = just("\\")
            .then(choice((
                just("\\"),
                just("/"),
                just("\""),
                just("b").to("\x08"),
                just("f").to("\x0C"),
                just("n").to("\n"),
                just("r").to("\r"),
                just("t").to("\t"),
                just("u").ignore_then(text::digits(16).exactly(4).to_slice().validate(
                    |digits, e, emitter| {
                        format!("{}",
                        char::from_u32(u32::from_str_radix(digits, 16).unwrap()).unwrap_or_else(
                            || {
                                emitter.emit(Default::default());
                                '\u{FFFD}' // unicode replacement character
                            },
                        )
                        ).as_str()
                    },
                )),
            )))
            .ignored(); */

        let string = just("\"").ignore_then(
            any()
                .and_is(just("\"").not())
                .repeated()
                .collect()
        ).then_ignore(just("\""))
        .map(Tkn::Str);
    
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
        .or(string)
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
