use ast::parser;
use chumsky::prelude::*;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

mod ast;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let src = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let ast = parser().parse(&src).unwrap();
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start;
    println!("{:#?}", ast);
    println!("time: {:?}", time);
}
