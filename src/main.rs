#![allow(non_snake_case)]
use std::fs;

mod ast;

fn main() {
    let contents = fs::read_to_string("./source/test.nv")
        .expect("Should have been able to read the file");

    let ast = ast::create(&contents);

    println!("{:#?}", ast);
}