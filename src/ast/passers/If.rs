use crate::ast::types::Expr;

pub fn pass(ast: &mut Vec<Expr>){
    if ast.len() > 0 {
        for i in 0..(ast.len() - 1) {
            if let Some(_) = ast.get(i) {
                match  ast[i].clone() {
                    Expr::If { condition, body: mut valid } => {
                        pass(&mut valid);
                        if i < ast.len() - 1 {
                            match  ast[i+1].clone() {
                                Expr::Else { body: mut invalid } => {
                                    ast[i+1] = Expr::Empty();
                                    pass(&mut invalid);
                                    (*ast)[i] = Expr::ElIf { condition, valid, invalid }
                                },
                                _ => {}
                            }
                        }
                    },
                    Expr::While { condition: _, mut body } => {
                        pass(&mut body);
                    }
                    _ => {}
                }
            }
        }
    }
}