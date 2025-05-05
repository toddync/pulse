use super::types::{Expr, Op, Spanned, Value};
use crate::interpreter::env::EnvPtr;
mod env;
mod operators;

#[derive(Clone)]
pub enum Res<'a> {
    V(Value<'a>),
    E(String),
}

pub fn run<'a>(ast: Vec<Box<Spanned<Expr<'a>>>>) {
    let globals: EnvPtr = env::new(None);

    for expr in ast {
        match exec(expr, globals.clone()) {
            Res::E(msg) => println!("{}", msg),
            Res::V(_) => {}
        }
    }

    //println!("{:#?}", globals);
}

fn exec<'a>(expr: Box<Spanned<Expr<'a>>>, env: EnvPtr<'a>) -> Res<'a> {
    match expr.clone().0 {
        Expr::Let(name, value) => {
            let r = exec(value, env.clone());
            match r {
                Res::V(value) => {
                    env::define(env.clone(), name, value);
                    Res::V(Value::Undefined)
                }
                Res::E(e) => Res::E(e),
            }
        }
        Expr::Assign(name, value) => {
            let r = exec(value, env.clone());
            match r {
                Res::V(value) => {
                    env::set(&env.clone(), name, value);
                    Res::V(Value::Undefined)
                }
                Res::E(e) => Res::E(e),
            }
        }
        Expr::Var(name) => {
            let r = env::get(env.clone(), name);
            match r {
                Some(value) => Res::V(value),
                None => Res::E("Undefined variable.".into()),
            }
        }

        Expr::BnOp(left, op, right) => {
            let ls = exec(left, env.clone());
            let rs = exec(right, env.clone());
            match (ls, rs) {
                (Res::V(l), Res::V(r)) => match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => l / r,
                    Op::And => Res::V(Value::Bool(l.is_truthy() && r.is_truthy())),
                    Op::Or => Res::V(Value::Bool(l.is_truthy() || r.is_truthy())),
                    Op::Eq => Res::V(Value::Bool(l == r)),
                    Op::Neq => Res::V(Value::Bool(l != r)),
                    Op::Gt => Res::V(Value::Bool(l > r)),
                    Op::Gte => Res::V(Value::Bool(l >= r)),
                    Op::Lt => Res::V(Value::Bool(l < r)),
                    Op::Lte => Res::V(Value::Bool(l <= r)),

                    _ => {
                        println!("{:#?}", expr);
                        Res::E("Not implemented".into())
                    }
                },

                (Res::E(e), _) => Res::E(e),
                (_, Res::E(e)) => Res::E(e),
            }
        }

        Expr::UnOp(op, value) => {
            let v = exec(value, env.clone());
            match v {
                Res::V(v) => match op {
                    Op::Neg => -v,
                    Op::Not => Res::V(Value::Bool(!v.is_truthy())),
                    _ => {
                        println!("{:#?}", expr);
                        Res::E("Not implemented".into())
                    }
                },
                Res::E(msg) => Res::E(msg),
            }
        }

        Expr::Block(stmts) => {
            let mut r = Res::V(Value::Undefined);
            for stmt in stmts {
                r = exec(stmt, env.clone());
            }
            r
        }

        Expr::Fn(name, args, body) => {
            let r = env::define(env.clone(), name, Value::Fn(args, body));
            match r {
                None => Res::V(Value::Undefined),
                Some(_) => Res::E("Function already defined.".into()),
            }
        }
        Expr::Return(value) => exec(value, env.clone()),

        Expr::If(cond, then, els) => {
            let c = exec(cond, env.clone());
            let env_ = env::new(Some(env.clone()));
            match c {
                Res::V(v) => {
                    if v.is_truthy() {
                        exec(then, env_.clone())
                    } else {
                        exec(els, env_.clone())
                    }
                }
                Res::E(e) => Res::E(e),
            }
        }

        Expr::While(condition, body) => {
            let mut result = Res::V(Value::Undefined);
            loop {
                match exec(condition.clone(), env.clone()) {
                    Res::V(v) => {
                        if !v.is_truthy() {
                            break;
                        }
                        let loop_env = env::new(Some(env.clone()));
                        result = exec(body.clone(), loop_env);
                        if let Res::E(_) = result {
                            break;
                        }
                    }
                    Res::E(_) => break,
                }
            }
            result
        }

        Expr::Call(r#fn, args) => {
            let r = exec(r#fn, env.clone());
            match r {
                Res::V(v) => match v {
                    Value::Fn(params, body) => {
                        let env_ = env::new(Some(env.clone()));

                        // Evaluate all call arguments
                        let mut evaluated_args = Vec::new();
                        for arg in args {
                            match exec(arg, env.clone()) {
                                Res::V(v) => evaluated_args.push(v),
                                Res::E(e) => return Res::E(e),
                            }
                        }

                        // Assign arguments to parameter names
                        for (i, param) in params.iter().enumerate() {
                            let value = evaluated_args.get(i).cloned().unwrap_or(Value::Undefined);
                            env::define(env_.clone(), param.clone(), value);
                        }

                        exec(body, env_.clone())
                    }
                    _ => Res::E("Not a function.".into()),
                },
                Res::E(msg) => Res::E(msg),
            }
        }
        Expr::ReservedCall(name, args) => exec_reserved_call(name, args, env.clone()),
        Expr::Val(n) => Res::V(n),
        _ => {
            println!("{:#?}", expr);
            Res::E("Not implemented".into())
        }
    }
}

fn exec_reserved_call<'a>(
    name: &'a str,
    args: Vec<Box<Spanned<Expr<'a>>>>,
    env: EnvPtr<'a>,
) -> Res<'a> {
    match name {
        "print" => {
            let len = args.len() - 1;
            for (i, arg) in args.into_iter().enumerate() {
                let r = exec(arg, env.clone());
                match r {
                    Res::V(v) => {
                        if i < len {
                            print!("{} ", v);
                        } else {
                            print!("{}", v);
                        }
                    }
                    Res::E(_) => {}
                }
            }
            println!();
            Res::V(Value::Undefined)
        }
        _ => {
            //println!("{:#?}", expr);
            Res::E("Not implemented".into())
        }
    }
}
