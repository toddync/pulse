use crate::ast::types::Expr;
use super::TrackTable;

pub fn pass(ast: &mut Vec<Expr>, memo: &mut TrackTable){  
    let mut i = 0;
    while i < ast.len() - 1 {
        match  ast[i].to_owned() {
            Expr::Assign { name, value, line: _} | Expr::VarDec { name, value, line: _} => {
                match memo.get(&name) {
                    Some(_) => { },
                    None => { memo.insert(name.clone(), false); }
                };
                sniff(&value, memo);
            },
            Expr::ElIf { condition, mut valid, mut invalid } => {
                sniff(&condition, memo);
                pass(&mut valid, memo);
                pass(&mut invalid, memo);
            },
            Expr::If { condition, mut body } => {
                sniff(&condition, memo);
                pass(&mut body, memo);
            },
            Expr::While { condition, mut body } => {
                sniff(&condition, memo);
                pass(&mut body, memo);
            },
            Expr::FnCall { name: _, params } => {
                for param in params {
                    sniff(&param, memo)
                }
            },
            Expr::Empty() => { ast.remove(i); continue },
            _ => { }
        }
        i +=1 ;
    }

    i = 0;
    while i < ast.len() - 1 {
        match  ast[i].to_owned() {
            Expr::Assign { name, value: _, line: _ } | Expr::VarDec { name, value: _, line: _ } => {
                match memo.get(&name) {
                    Some(x) => if !x { ast.remove(i); continue },
                    None => { }
                };
            },
            _ => { }
        };
        i +=1 ;
    }
}

fn sniff(expr: &Expr, memo: &mut TrackTable) {
    match expr.to_owned() {
        Expr::BinaryOp { left, op: _, right } => {
            sniff(&left, memo);
            sniff(&right, memo);
        }
        Expr::Variable { name, line: _ } => {
            memo.insert(name.clone(), true);
        }
        _ => { }
    }
}