use crate::ast::types::Expr;
use super::TrackTable;


pub fn pass(ast: &mut [Expr], memo: &mut TrackTable) {
    for i in 0..ast.len() - 1 {
        match  ast[i].to_owned() {
            Expr::Assign { name, value, line} => {
                match memo.get(&name) {
                    Some(_) => { },
                    None => {
                        memo.insert(name.clone(), false);
                        ast[i] = Expr::VarDec { name, value, line }
                    }
                };
            },
            Expr::ElIf { condition: _, mut valid, mut invalid } => {
                pass(&mut valid, memo);
                pass(&mut invalid, memo);
            },
            Expr::If { condition: _, mut body } => {
                pass(&mut body, memo);
            },
            Expr::While { condition: _, mut body } => {
                pass(&mut body, memo);
            },
            _ => { }
        }
    }
}