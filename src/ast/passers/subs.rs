use crate::ast::{passers::fold::fold, types::Expr};
use std::collections::HashMap;

pub type SymbolTable = HashMap<String, (Expr, usize)>;

pub fn pass(ast: &mut [Expr], memo: &mut SymbolTable){  
    for i in 0..ast.len() {
        match  ast[i].to_owned() {
            Expr::Assign { name, mut value, line} => {
                subs(&mut value, memo);
                value = Box::new(fold(&value));

                if is_literal(&value) {
                    memo.insert(name.clone(), (*value.clone(), line));
                }

                ast[i] = Expr::Assign { name, value, line }
            },
            Expr::ElIf { mut condition, mut valid, mut invalid } => {
                subs(&mut condition, memo);
                condition = Box::new(fold(&condition));

                pass(&mut valid, memo);
                pass(&mut invalid, memo);

                *memo = SymbolTable::new();

                ast[i] = Expr::ElIf { condition, valid, invalid }
            },
            Expr::If { mut condition, body } => {
                subs(&mut condition, memo);
                condition = Box::new(fold(&condition));

                ast[i] = Expr::If { condition, body }
            },

            Expr::FnCall { name, mut params } => {
                params = params
                    .into_iter()
                    .map(|mut param| {
                        subs(&mut param, memo);
                        fold(&param)
                    })
                    .collect::<Vec<Expr>>();

                ast[i] = Expr::FnCall { name, params }
            }
            _ => {}
        }
    }
}

fn is_literal(expr: &Expr) -> bool {
    match expr {
        Expr::Number(_) => true,
        Expr::String(_) => true,
        Expr::Float(_) => true,
        Expr::Bool(_) => true,
        _ => false
    }
}

fn subs(expr: &mut Expr, memo: &SymbolTable) {
    match expr.to_owned() {
        Expr::BinaryOp { mut left, op, mut right } => {
            subs(&mut left, memo);
            subs(&mut right, memo);
            *expr = Expr::BinaryOp { left, op, right };
        },
        Expr::Variable { name, line } => {
            if memo.contains_key(&name) {
                let (val, tl) = memo.get(&name).unwrap().to_owned();
                if tl >= line {
                    *expr = val;
                }
            }
        }
        _ => {}
    }
}