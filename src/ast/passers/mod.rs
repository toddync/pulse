#![allow(non_snake_case)]
use std::collections::HashMap;
use super::types::Expr;

type SymbolTable = HashMap<String, (Expr, usize)>;
type TrackTable = HashMap<String, bool>;

mod If;
mod subs;
mod fold;
mod unwrap;
mod cleanup;
mod variables;

pub fn run(ast: &mut Vec<Expr>) {
    fold::pass(ast);
    If::pass(ast);
    subs::pass(ast, &mut SymbolTable::new());
    unwrap::pass(ast);
    subs::pass(ast, &mut SymbolTable::new());
    variables::pass(ast, &mut TrackTable::new());

    cleanup::pass(ast, &mut TrackTable::new());
}