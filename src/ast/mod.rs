mod types;
mod help;
mod lex;
mod rpn;

use types::{Expr, Token, TokenKind};
use rpn::{rpn_to_expr, rpn};
use help::is_literal;
use lex::lexer;


pub fn create(contents: &str) -> Vec<Expr> {
    ast(&lexer(contents))
}

pub fn ast(tokens: &Vec<Token>) -> Vec<Expr> {
    let mut instructions: Vec<Expr> = vec![];

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];

        if token.kind == TokenKind::Identifier &&  tokens[i+1].kind == TokenKind::Assign{
            let mut expression = String::new();
            let mut last_tkn = &tokens[i+1];
            i += 2;
            while i < tokens.len() {
                let piece = &tokens[i];

                if  piece.kind == TokenKind::Semicolon ||
                    (piece.kind == TokenKind::Nl && !( last_tkn.kind == TokenKind::Operator || last_tkn.kind == TokenKind::Delimiter)) {
                        break;
                } else if is_literal(&last_tkn.kind) && (is_literal(&piece.kind) || piece.kind == TokenKind::Identifier) {
                    panic!("expected a operator")
                } else if piece.kind == TokenKind::EOF && last_tkn.kind == TokenKind::Operator{
                    panic!("unexpected end of file");
                } else if piece.kind == TokenKind::EOF {
                    break;
                }

                if piece.kind != TokenKind::Nl {
                    expression.push_str(&piece.value);
                    expression.push(' ');
                    last_tkn = piece;
                }

                i += 1;
            }
            
            let value = rpn_to_expr(rpn(&expression)).unwrap();
            instructions.push(Expr::Assign { name: token.value.clone(), value: Box::new(value) });
        }

        if token.kind == TokenKind::Keyword {
            if token.value == "if" {
                let mut condition = String::new();
                // let mut last_tkn = &tokens[i+1];
                i += 1;

                while i < tokens.len() {
                    let piece = &tokens[i];

                    if piece.kind == TokenKind::Delimiter && piece.value == "{" {
                        break;
                    }

                    if piece.kind != TokenKind::Nl {
                        condition.push_str(&piece.value);
                        condition.push(' ');
                        // last_tkn = piece;
                    }

                    i += 1;
                }

                instructions.push(Expr::If {
                    condition: Box::new(rpn_to_expr(rpn(&condition)).unwrap()),
                    body: vec![]
                })
            }
        }

        i += 1;
    }

    instructions
}