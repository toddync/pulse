#![allow(non_snake_case)]
use subs::SymbolTable;

use super::types::Expr;

mod If;
mod subs;
mod fold;
mod unwrap;

pub fn run(ast: &mut Vec<Expr>) {
    fold::pass(ast);
    If::pass(ast);
    subs::pass(ast, &mut SymbolTable::new());
    unwrap::pass(ast);
    subs::pass(ast, &mut SymbolTable::new());
}