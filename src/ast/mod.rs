mod lex;
mod passers;
mod rpn;
pub mod types;

use lex::lexer;
use rpn::{rpn, rpn_to_expr};
use types::{Expr, Token, TokenKind};

pub fn create(contents: &str) -> Vec<Expr> {
    let mut res = ast(&lexer(contents), false);

    passers::pass(&mut res.0);

    res.0
}

fn ast(tokens: &[Token], single: bool) -> (Vec<Expr>, usize) {
    let mut instructions: Vec<Expr> = vec![];

    let mut i = 0;
    let mut line = 0;
    while (!single && i < tokens.len()) || (single && i < 1) {
        let token = &tokens[i];
        if token.value == "}" || token.kind == TokenKind::Eof {
            break;
        }

        if token.kind == TokenKind::Identifier && tokens[i + 1].kind == TokenKind::Assign {
            let variable_name = token.value.clone();
            i += 2;

            let expression = collect_expression_until(tokens, &mut i, |t, i| {
                t.kind == TokenKind::Semicolon || t.kind == TokenKind::Eof
                    || (t.kind == TokenKind::Nl
                        && !(matches!(tokens.get(i - 1), Some(prev) if prev.kind == TokenKind::Operator || (prev.kind == TokenKind::Delimiter && (prev.value == "(")))))
            }).expect("Error collecting expression");

            let value = rpn_to_expr(rpn(&expression)).unwrap();
            instructions.push(Expr::Assign {
                name: variable_name,
                value: Box::new(value),
                line,
            });

            if tokens
                .get(i)
                .map_or(false, |t| t.kind == TokenKind::Semicolon)
            {
                i += 1;
            }
        } else if token.kind == TokenKind::Keyword {
            if token.value == "if" {
                i += 1;
                let condition = collect_expression_until(tokens, &mut i, |t, _| {
                    t.kind == TokenKind::Delimiter && t.value == "{"
                })
                .expect("Error collecting expression");

                let res = ast(&tokens[i..], false);
                i += res.1;

                instructions.push(Expr::If {
                    condition: Box::new(rpn_to_expr(rpn(&condition)).unwrap()),
                    body: res.0,
                });

                if tokens[i + 1].value == "else" {
                    i += 1;

                    let mut res = ast(&tokens[i..], true);
                    i += res.1;

                    instructions.append(&mut res.0);
                }
            } else if token.value == "else" {
                i += 1;
                let mut s = true;
                if tokens[i].value != "if" {
                    i += 1;
                    s = false
                }
                let res = ast(&tokens[i..], s);
                i += res.1;

                instructions.push(Expr::Else { body: res.0 });
            } else if token.value == "while" {
                i += 1;
                let condition = collect_expression_until(tokens, &mut i, |t, _| {
                    t.kind == TokenKind::Delimiter && t.value == "{"
                })
                .expect("Error collecting expression");

                let res = ast(&tokens[i..], false);
                i += res.1;

                instructions.push(Expr::While {
                    condition: Box::new(rpn_to_expr(rpn(&condition)).unwrap()),
                    body: res.0,
                });
            } else if token.value == "for" {
                i += 1;
                let condition = collect_expression_until(tokens, &mut i, |t, _| {
                    t.value == "{" || t.value == "in"
                })
                .expect("Error collecting expression");

                // let res = ast(&tokens[i..]);
                // i += res.1;

                println!("\n{:#?}\n", rpn_to_expr(rpn(&condition)).unwrap());

                // instructions.push(Expr::For {
                //     domain: Box::new(rpn_to_expr(rpn(&condition)).unwrap()),
                //     body: res.0,
                // });
            }
        } else if token.kind == TokenKind::Identifier {
            let function_name = token.value.clone();
            if tokens.get(i + 1).map_or(false, |t| t.value == "(") {
                i += 2;
                let mut params = vec![];

                while i < tokens.len() && tokens[i].value != ")" {
                    let arg_tokens = collect_expression_until(tokens, &mut i, |t, _| {
                        t.value == "," || t.value == ")"
                    })
                    .unwrap();
                    let arg_expr = rpn_to_expr(rpn(&arg_tokens)).unwrap();
                    params.push(arg_expr);

                    if tokens[i].value == "," {
                        i += 1;
                    }
                }

                if tokens[i].value != ")" {
                    panic!("Unmatched parentheses in function call");
                }
                i += 1;

                instructions.push(Expr::FnCall {
                    name: function_name,
                    params,
                });
            }
        } else if token.kind == TokenKind::Nl {
            line += 1;
        }
        i += 1;
    }

    (instructions, i)
}

fn collect_expression_until<F>(
    tokens: &[Token],
    i: &mut usize,
    is_delimiter: F,
) -> Result<String, String>
where
    F: Fn(&Token, usize) -> bool,
{
    let mut expression = String::new();
    let mut parentheses = 0;

    while *i < tokens.len() && (!is_delimiter(&tokens[*i], *i) || parentheses > 0) {
        let token = &tokens[*i];

        match token.value.as_str() {
            "(" => parentheses += 1,
            ")" => {
                if parentheses == 0 {
                    return Err("Unmatched closing parenthesis".to_string());
                }
                parentheses -= 1;
            }
            _ => {}
        }

        if token.kind != TokenKind::Nl {
            expression.push_str(&token.value);
            expression.push(' ');
        }

        *i += 1;
    }

    if parentheses > 0 {
        return Err("Unmatched opening parenthesis".to_string());
    }

    Ok(expression.trim().to_string())
}
