use crate::ast::types::Expr;

pub fn pass(ast: &mut Vec<Expr>){  
    for i in 0..ast.len() {
        match  ast[i].to_owned() {
            Expr::If { condition, mut body } => {
                pass(&mut body);
                match *condition {
                    Expr::Bool(b) => {
                        if b {
                            ast[i] = Expr::Empty();
                            ast.splice(i..i, body);
                        }
                    }
                    _ => {}
                }
            },
            Expr::ElIf { condition, mut valid, mut invalid } => {
                match *condition {
                    Expr::Bool(b) => {
                        ast[i] = Expr::Empty();
                        let e = i+1;
                        if b {
                            pass(&mut valid);
                            ast.splice(e..e, valid);
                        } else {
                            pass(&mut invalid);
                            ast.splice(e..e, invalid);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}