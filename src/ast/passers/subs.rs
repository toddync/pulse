use super::TrackTable;
use crate::ast::{passers::fold::fold, types::Expr};
use std::collections::HashMap;

pub fn pass(ast: &mut [Expr], memo: &mut TrackTable<Expr>) {
    for node in ast {
        match node.clone() {
            Expr::Assign {
                name,
                mut value,
                line,
            } => {
                subs(&mut value, memo);
                value = Box::new(fold(&value));

                if is_literal(&value) {
                    memo.insert(name.clone(), *value.clone());
                }

                *node = Expr::Assign {
                    name: name.to_string(),
                    line,
                    value,
                }
            }
            Expr::ElIf {
                mut condition,
                mut valid,
                mut invalid,
            } => {
                subs(&mut condition, memo);
                condition = Box::new(fold(&condition));

                pass(&mut valid, memo);
                pass(&mut invalid, memo);

                *memo = HashMap::new();

                *node = Expr::ElIf {
                    condition,
                    valid,
                    invalid,
                }
            }
            Expr::If {
                mut condition,
                body,
            } => {
                subs(&mut condition, memo);
                condition = Box::new(fold(&condition));

                *node = Expr::If {
                    condition,
                    body: body.to_vec(),
                }
            }

            Expr::FnCall { name, mut params } => {
                params = params
                    .into_iter()
                    .map(|mut param| {
                        subs(&mut param, memo);
                        fold(&param)
                    })
                    .collect::<Vec<Expr>>();

                *node = Expr::FnCall {
                    name: name.to_string(),
                    params,
                }
            }
            _ => {}
        }
    }
}

fn is_literal(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Number(_) | Expr::String(_) | Expr::Float(_) | Expr::Bool(_)
    )
}

fn subs(expr: &mut Expr, memo: &TrackTable<Expr>) {
    match expr.to_owned() {
        Expr::BinaryOp {
            mut left,
            op,
            mut right,
        } => {
            subs(&mut left, memo);
            subs(&mut right, memo);
            *expr = Expr::BinaryOp { left, op, right };
        }
        Expr::Variable(name) => {
            if memo.contains_key(&name) {
                let val = memo.get(&name).unwrap().to_owned();
                *expr = val;
            }
        }
        _ => {}
    }
}
