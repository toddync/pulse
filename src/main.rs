use ast::{lex, parse, show_errors};
use std::{env, fs, process::exit};

mod ast;
mod interpreter;
pub mod types;

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let source = fs::read_to_string(&filename).unwrap();

    let (tokens, lex_errs) = lex(&source);
    let tokens = tokens.unwrap_or_else(|| {
        show_errors(lex_errs, filename.clone(), &source);
        exit(1);
    });

    let (ast, parse_errs) = parse(&tokens, &source);
    let ast = ast.unwrap_or_else(|| {
        show_errors(parse_errs, filename.clone(), &source);
        exit(1);
    });

    //println!("{:#?}", ast);

    interpreter::run(ast);
}
