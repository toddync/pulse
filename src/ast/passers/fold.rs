use crate::ast::types::{Expr, Operator};

pub fn pass(ast: &mut [Expr]){
    for i in 0..ast.len() {
        match  ast[i].to_owned() {
            Expr::Assign { name, value, line} => {
                ast[i] = Expr::Assign { 
                    name: name.to_string(),
                    value: Box::new(fold(&value)),
                    line
                }
            },
            Expr::If { condition, mut body } => {
                pass(&mut body);
                ast[i] = Expr::If { 
                    condition: Box::new(fold(&condition)),
                    body,
                }
            }
            _ => {}
        }
    }
}

pub fn fold(expr: &Expr) -> Expr {
    if let Expr::BinaryOp { mut left, op, mut right } = expr.to_owned() {
        left = Box::new(fold(&*left));
        right = Box::new(fold(&*right));

        if is_literal(&left) && is_literal(&right) {
            return match op {
                Operator::Add => match (*left.clone(), *right.clone()) {
                    (Expr::String(l), Expr::String(r)) => Expr::String(format!("{}{}", l, r)),
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l + r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Float(l + r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Float(l as f64 + r),
                    (Expr::Number(l), Expr::String(r)) => Expr::String(format!("{}{}", l, r)),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Float(l + r  as f64),
                    (Expr::Float(l), Expr::String(r)) => Expr::String(format!("{}{}", l, r)),
                    (Expr::String(l), Expr::Number(r)) => Expr::String(format!("{}{}", l, r)),
                    (Expr::String(l), Expr::Float(r)) => Expr::String(format!("{}{}", l, r)),

                    _ => expr.to_owned()
                },
                Operator::Subtract => match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l - r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Float(l - r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Float(l as f64 - r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Float(l - r  as f64),
                    _ => expr.to_owned()
                },
                Operator::Multiply => match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l * r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Float(l * r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Float(l as f64 * r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Float(l * r  as f64),
                    _ => expr.to_owned()
                },
                Operator::Divide => match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l / r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Float(l / r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Float(l as f64 / r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Float(l / r  as f64),
                    _ => expr.to_owned()
                },
                Operator::Modulus => match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l % r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Float(l % r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Float(l as f64 % r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Float(l % r  as f64),
                    _ => expr.to_owned()
                },

                //* --- *//

                Operator::Greater => match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Bool(l > r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Bool(l > r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Bool(l as f64 > r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Bool(l > r  as f64),
                    _ => expr.to_owned()
                },
                Operator::Less => match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Bool(l < r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Bool(l < r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Bool((l as f64) < r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Bool(l < r  as f64),
                    _ => expr.to_owned()
                },
                Operator::Equals => match (*left.clone(), *right.clone()) {
                    (Expr::String(l), Expr::String(r)) => Expr::Bool(l == r),
                    (Expr::Number(l), Expr::Number(r)) => Expr::Bool(l == r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Bool(l == r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Bool(l as f64 == r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Bool(l == r  as f64),
                    _ => expr.to_owned()
                },
                Operator::Different => match (*left.clone(), *right.clone()) {
                    (Expr::String(l), Expr::String(r)) => Expr::Bool(l != r),
                    (Expr::Number(l), Expr::Number(r)) => Expr::Bool(l != r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Bool(l != r),
                    (Expr::Number(l), Expr::Float(r)) => Expr::Bool(l as f64 != r),
                    (Expr::Float(l), Expr::Number(r)) => Expr::Bool(l != r  as f64),
                    _ => expr.to_owned()
                },

                Operator::And => match (as_bool(&left), as_bool(&right)) {
                    (true, _) => Expr::Bool(true),
                    (false, _) => Expr::Bool(false),
                }
                Operator::Or => match (as_bool(&left), as_bool(&right)) {
                    (true, _) => Expr::Bool(true),
                    (_, true) => Expr::Bool(true),
                    _ => Expr::Bool(false)
                }
                _ => expr.to_owned()
            }
        } else {
            return Expr::BinaryOp {
                left: left.to_owned(),
                op: op.to_owned(),
                right: right.to_owned()
            };
        }
    } else if let Expr::Not(mut n) = expr.to_owned() {
        n = Box::new(fold(&n));
        if is_literal(&n) {
            return Expr::Bool(match *n.clone() {
                Expr::Number(i) => i == 0,
                Expr::String(i) => i.len() > 0,
                Expr::Float(i) => i == f64::from(0),
                Expr::Bool(i) => i == false,
                _ => false
            })
        }
    }

    expr.to_owned()
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

fn as_bool(n: &Expr) -> bool {
    match n.clone() {
        Expr::Number(i) => i == 0,
        Expr::String(i) => i.len() > 0,
        Expr::Float(i) => i != f64::from(0),
        Expr::Bool(i) => i,
        _ => false
    }
}