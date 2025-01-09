use crate::ast::types::Expr;

pub fn pass(ast: &mut Vec<Expr>) {
    if !ast.is_empty() {
        for i in 0..(ast.len() - 1) {
            if ast.get(i).is_some() {
                match ast[i].clone() {
                    Expr::If {
                        condition,
                        body: mut valid,
                    } => {
                        pass(&mut valid);
                        if i < ast.len() - 1 {
                            if let Expr::Else { body: mut invalid } = ast[i + 1].clone() {
                                ast[i + 1] = Expr::Empty();
                                pass(&mut invalid);
                                (*ast)[i] = Expr::ElIf {
                                    condition,
                                    valid,
                                    invalid,
                                }
                            }
                        }
                    }
                    Expr::While {
                        condition: _,
                        mut body,
                    } => {
                        pass(&mut body);
                    }
                    _ => {}
                }
            }
        }
    }
}
