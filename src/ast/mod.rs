mod lex;
mod rpn;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Variable(String),
    String(String),
    Not(Box<Expr>),
    Number(i64),
    Float(f64),
    Bool(bool),
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    VarDec {
        name: String,
        value: Box<Expr>
    },
    If {
        condition: Box<Expr>,
        body: Vec<Expr>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    //* Math
    Subtract,
    Multiply,
    Modulus,
    Divide,
    Power,
    Add,

    //* Logic
    Different,
    LtEquals,
    GtEquals,
    Greater,
    Equals,
    Less,
    And,
    Or,
}

pub fn create(contents: &str) -> Vec<Expr> {
    ast(&lex::lexer(contents))
}

pub fn ast(tokens: &Vec<lex::Token>) -> Vec<Expr> {
    let mut instructions: Vec<Expr> = vec![];

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];

        if token.kind == lex::TokenKind::Identifier &&  tokens[i+1].kind == lex::TokenKind::Assign{
            let mut expression = String::new();
            let mut last_tkn = &tokens[i+1];
            i += 2;
            while i < tokens.len() {
                let piece = &tokens[i];

                if  piece.kind == lex::TokenKind::Semicolon ||
                    (piece.kind == lex::TokenKind::Nl && !( last_tkn.kind == lex::TokenKind::Operator || last_tkn.kind == lex::TokenKind::Delimiter)) {
                        break;
                } else if is_literal(&last_tkn.kind) && (is_literal(&piece.kind) || piece.kind == lex::TokenKind::Identifier) {
                    panic!("expected a operator")
                } else if piece.kind == lex::TokenKind::EOF && last_tkn.kind == lex::TokenKind::Operator{
                    panic!("unexpected end of file");
                } else if piece.kind == lex::TokenKind::EOF {
                    break;
                }

                if piece.kind != lex::TokenKind::Nl {
                    expression.push_str(&piece.value);
                    expression.push(' ');
                    last_tkn = piece;
                }

                i += 1;
            }
            
            let value = parse_postfix_to_ast(rpn::rpn(&expression)).unwrap();
            instructions.push(Expr::VarDec { name: token.value.clone(), value: Box::new(value) });
        }

        if token.kind == lex::TokenKind::Keyword {
            if token.value == "if" {
                let mut condition = String::new();
                // let mut last_tkn = &tokens[i+1];
                i += 1;

                while i < tokens.len() {
                    let piece = &tokens[i];

                    if piece.kind == lex::TokenKind::Delimiter && piece.value == "{" {
                        break;
                    }

                    if piece.kind != lex::TokenKind::Nl {
                        condition.push_str(&piece.value);
                        condition.push(' ');
                        // last_tkn = piece;
                    }

                    i += 1;
                }

                instructions.push(Expr::If {
                    condition: Box::new(parse_postfix_to_ast(rpn::rpn(&condition)).unwrap()),
                    body: vec![]
                })
            }
        }

        i += 1;
    }

    instructions
}

fn parse_postfix_to_ast(tokens: Vec<String>) -> Result<Expr, String> {
    let mut stack: Vec<Expr> = Vec::new();

    if tokens.len() == 1 {
        let token = tokens.into_iter().next().unwrap();
        if let Ok(value) = token.parse::<i64>() {
            return Ok(Expr::Number(value));
        } else if let Ok(value) = token.parse::<f64>() {
            return Ok(Expr::Float(value));
        } else if token.starts_with('"') && token.ends_with('"') {
            let trimmed = token.trim_matches('"').to_string(); //* Remove surrounding quotes
            return Ok(Expr::String(trimmed));
        } else if token.to_lowercase() == "true" {
            return Ok(Expr::Bool(true));
        } else if token.to_lowercase() == "false" {
            return Ok(Expr::Bool(false));
        } else {
            return Ok(Expr::Variable(token));
        }
    }

    for token in tokens {
        match token.to_lowercase().as_str() {
            "+" => {
                let right = stack.pop().ok_or("Stack underflow for '+' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '+' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Add,
                    right: Box::new(right),
                });
            }
            "-" => {
                let right = stack.pop().ok_or("Stack underflow for '-' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '-' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Subtract,
                    right: Box::new(right),
                });
            }
            "/" => {
                let right = stack.pop().ok_or("Stack underflow for '/' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '/' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Divide,
                    right: Box::new(right),
                });
            }
            "*" => {
                let right = stack.pop().ok_or("Stack underflow for '*' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '*' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Multiply,
                    right: Box::new(right),
                });
            }
            "%" => {
                let right = stack.pop().ok_or("Stack underflow for '%' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '%' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Modulus,
                    right: Box::new(right),
                });
            }
            "^" => {
                let right = stack.pop().ok_or("Stack underflow for '^' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '^' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Power,
                    right: Box::new(right),
                });
            }

            ">" => {
                let right = stack.pop().ok_or("Stack underflow for '>' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '>' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Greater,
                    right: Box::new(right),
                });
            }
            "<" => {
                let right = stack.pop().ok_or("Stack underflow for '<' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '<' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Less,
                    right: Box::new(right),
                });
            }
            ">=" => {
                let right = stack.pop().ok_or("Stack underflow for '>=' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '>=' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::GtEquals,
                    right: Box::new(right),
                });
            }
            "<=" => {
                let right = stack.pop().ok_or("Stack underflow for '<=' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '<=' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::LtEquals,
                    right: Box::new(right),
                });
            }
            "==" => {
                let right = stack.pop().ok_or("Stack underflow for '==' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '==' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Equals,
                    right: Box::new(right),
                });
            }
            "!=" => {
                let right = stack.pop().ok_or("Stack underflow for '!=' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '!=' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Different,
                    right: Box::new(right),
                });
            }
            "&" => {
                let right = stack.pop().ok_or("Stack underflow for '&' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '&' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::And,
                    right: Box::new(right),
                });
            }
            "and" => {
                let right = stack.pop().ok_or("Stack underflow for 'and' operator")?;
                let left = stack.pop().ok_or("Stack underflow for 'and' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::And,
                    right: Box::new(right),
                });
            }
            "|" => {
                let right = stack.pop().ok_or("Stack underflow for '|' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '|' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Or,
                    right: Box::new(right),
                });
            }
            "or" => {
                let right = stack.pop().ok_or("Stack underflow for 'or' operator")?;
                let left = stack.pop().ok_or("Stack underflow for 'or' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Or,
                    right: Box::new(right),
                });
            }
            "!" => {
                let operand = stack.pop().ok_or("Stack underflow for '!' operator")?;
                stack.push(Expr::Not(Box::new(operand)));
            }
            
            _ => {
                //* Handle operands: try parsing as a number, check if it's a bool or string, otherwise assume it's a variable
                if let Ok(number) = token.parse::<i64>() {
                    stack.push(Expr::Number(number));
                } else if let Ok(number) = token.parse::<f64>() {
                    stack.push(Expr::Float(number));
                } else if token == "true" {
                    stack.push(Expr::Bool(true));
                } else if token == "false" {
                    stack.push(Expr::Bool(false));
                } else /* if token.chars().all(char::is_alphanumeric) */ {
                    stack.push(Expr::Variable(token));
                }/*  else {
                    return Err(format!("Invalid token: {}", token));
                } */
            }
        }
    }

    //* The final AST should be the only element left on the stack
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("Invalid postfix expression: leftover elements in stack".to_string())
    }
}


fn is_literal(kind: &lex::TokenKind) -> bool {
    if  kind == &lex::TokenKind::String ||
        kind == &lex::TokenKind::Number ||
        kind == &lex::TokenKind::Float  ||
        kind == &lex::TokenKind::Bool
    {
        return true;
    } else {
        return false;
    }
}