#![allow(non_snake_case)]
use super::types::Expr;
use std::collections::HashMap;

type TrackTable<T> = HashMap<String, T>;

mod If;
mod cleanup;
mod fold;
mod subs;
mod unwrap;
mod variables;

pub fn pass(ast: &mut Vec<Expr>) {
    fold::pass(ast);
    If::pass(ast);
    subs::pass(ast, &mut HashMap::new());
    unwrap::pass(ast);
    subs::pass(ast, &mut HashMap::new());
    variables::pass(ast, &mut HashMap::new());

    cleanup::pass(ast, &mut HashMap::new());
}
