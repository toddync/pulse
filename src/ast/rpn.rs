use super::types::{Expr, Operator};

pub fn rpn(expression: &str) -> Vec<String> {
    let mut rpn: Vec<String> = Vec::new(); // Resulting RPN expression
    let mut op_stack: Vec<String> = Vec::new(); // Stack to hold operators

    // Helper functions for operator properties
    fn is_operator(token: &str) -> bool {
        matches!(token, 
            "!" |
            "^" |
            "%" |
            "*" | "/" |
            "+" | "-" |
            ">" | "<" |
            ">=" | "<=" |
            "==" | "!=" |
            "&" | "and" |
            "|" | "or" 
        )
    }

    fn is_right_associative(op: &str) -> bool {
        matches!(op, "^" | "!")
    }

    fn precedence(op: &str) -> i32 {
        match op {
            "!" => 8,        // Unary negation has the highest precedence
            "^" => 7,
            "*" | "/" | "%" => 6,
            "+" | "-" => 5,
            ">" | "<" => 4,
            ">=" | "<=" => 3,
            "!=" | "==" => 2,
            "&" | "and" => 1,
            "|" | "or" => 0,
            _ => -1,
        }
    }

    // Tokenize the expression
    let mut prev_token: Option<String> = None; // Keep track of the previous token

    let binding = expression.to_string();
    let primitives: Vec<&str> = binding.split(" ").collect();
    let mut i = 0;
    while i < primitives.len() {
        let mut token = primitives[i].to_string();
        match token.as_str() {
            "(" => {
                op_stack.push(token.to_string());
                prev_token = Some(token.to_string());
            }
            ")" => {
                // Pop operators until "(" is found
                while let Some(top) = op_stack.last() {
                    if top == "(" {
                        break;
                    }
                    rpn.push(op_stack.pop().unwrap());
                }
                op_stack.pop(); // Remove the "("
                prev_token = Some(token.to_string());
            }
            _ if is_operator(&token) => {
                // Handle unary negation
                if token == "-" {
                    if prev_token.is_none() || prev_token.as_ref().map_or(false, |t| is_operator(t) || t == "(") {
                        op_stack.push("!".to_string());
                        prev_token = Some("!".to_string());
                        continue;
                    }
                }

                // Handle operator precedence and associativity
                while let Some(top) = op_stack.last() {
                    if is_operator(top) {
                        if (is_right_associative(&token) && precedence(&token) < precedence(top))
                            || (!is_right_associative(&token) && precedence(&token) <= precedence(top))
                        {
                            rpn.push(op_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                op_stack.push(token.to_string());
                prev_token = Some(token.to_string());
            }
            _ => {
                // Operand (number, variable, boolean, or string)
                if token.starts_with('"') {
                    i += 1;
                    token.push(' ');
                    while i < primitives.len() {
                        token.push_str(primitives[i]);
                        if primitives[i].ends_with('"') { break }
                        else { token.push(' ') }
                        i += 1;
                    }
                }
                rpn.push(token.clone());
                prev_token = Some(token);
            }
        }
        i += 1;
    }

    // Pop remaining operators onto the RPN stack
    while let Some(op) = op_stack.pop() {
        rpn.push(op);
    }

    rpn
}

pub fn rpn_to_expr(tokens: Vec<String>, line: usize) -> Result<Expr, String> {
    let mut stack: Vec<Expr> = Vec::new();

    if tokens.len() == 1 {
        let token = tokens.into_iter().next().unwrap();
        if let Ok(value) = token.parse::<i128>() {
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
            return Ok(Expr::Variable{name: token, line });
        }
    }

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        
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
                if let Ok(number) = token.parse::<i128>() {
                    stack.push(Expr::Number(number));
                } else if let Ok(number) = token.parse::<f64>() {
                    stack.push(Expr::Float(number));
                } else if token == "true" {
                    stack.push(Expr::Bool(true));
                } else if token == "false" {
                    stack.push(Expr::Bool(false));
                } else if token.starts_with('"') {
                    stack.push(Expr::String(tokens[i].trim_matches('"').to_string()));
                } else /* if token.chars().all(char::is_alphanumeric) */ {
                    stack.push(Expr::Variable{name: token.to_string(), line });
                }
            }
        }

        i += 1;
    }

    //* The final AST should be the only element left on the stack
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        println!("\n{:?}\n", tokens);
        println!("\n{:#?}\n", stack);
        Err("Invalid postfix expression: leftover elements in stack".to_string())
    }
}