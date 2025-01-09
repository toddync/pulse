use std::collections::HashMap;

use super::TrackTable;
use crate::ast::types::Expr;

pub fn pass(ast: &mut [Expr], memo: &mut TrackTable<usize>) {
    let mut i = 0;
    while i < ast.len() - 1 {
        match ast[i].to_owned() {
            Expr::VarDec {
                name,
                value,
                line: _,
            } => {
                memo.insert(name, i);
                sniff(&value, memo);
            }
            Expr::Assign {
                name,
                value,
                line: _,
            } => {
                if let Some(x) = memo.get(&name) {
                    ast[*x] = Expr::Empty();
                    memo.remove(&name);
                }

                memo.insert(name.clone(), i);
                sniff(&value, memo);
            }
            Expr::ElIf {
                condition,
                mut valid,
                mut invalid,
            } => {
                sniff(&condition, memo);
                pass(&mut valid, &mut HashMap::new());
                pass(&mut invalid, &mut HashMap::new());
            }
            Expr::If {
                condition,
                mut body,
            } => {
                sniff(&condition, memo);
                pass(&mut body, &mut HashMap::new());
            }
            Expr::While {
                condition,
                mut body,
            } => {
                sniff(&condition, memo);
                pass(&mut body, memo);
            }
            Expr::FnCall { name: _, params } => {
                for param in params {
                    sniff(&param, memo)
                }
            }
            _ => {}
        }
        i += 1;
    }

    i = 0;
    while i < ast.len() - 1 {
        match ast[i].to_owned() {
            Expr::Assign {
                name,
                value: _,
                line: _,
            }
            | Expr::VarDec {
                name,
                value: _,
                line: _,
            } => {
                if let Some(x) = memo.get(&name) {
                    if *x <= i {
                        ast[i] = Expr::Empty();
                    }
                }
            }
            _ => {}
        };
        i += 1;
    }
}

fn sniff(expr: &Expr, memo: &mut TrackTable<usize>) {
    match expr.to_owned() {
        Expr::BinaryOp { left, op: _, right } => {
            sniff(&left, memo);
            sniff(&right, memo);
        }
        Expr::Variable(name) => {
            memo.remove(&name);
        }
        _ => {}
    }
}
