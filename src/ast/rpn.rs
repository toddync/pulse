use super::types::{Expr, Operator, RpnExpr};

pub fn rpn(expression: &str) -> Vec<RpnExpr> {
    let mut rpn_stack: Vec<RpnExpr> = Vec::new();
    let mut op_stack: Vec<String> = Vec::new();

    fn is_operator(token: &str) -> bool {
        matches!(
            token,
            "!" | "^"
                | "%"
                | "*"
                | "/"
                | "+"
                | "-"
                | ">"
                | "<"
                | ">="
                | "<="
                | "=="
                | "!="
                | "&"
                | "and"
                | "|"
                | "or"
        )
    }

    fn is_right_associative(op: &str) -> bool {
        matches!(op, "^" | "!")
    }

    fn precedence(op: &str) -> i32 {
        match op {
            "!" => 8,
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

    let mut prev_token: Option<String> = None;

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
                while let Some(top) = op_stack.last() {
                    if top == "(" {
                        break;
                    }
                    rpn_stack.push(RpnExpr::Basic(op_stack.pop().unwrap()));
                }
                op_stack.pop();
                prev_token = Some(token.to_string());
            }
            _ if is_operator(&token) => {
                if token == "-"
                    && (prev_token.is_none()
                        || prev_token
                            .as_ref()
                            .map_or(false, |t| is_operator(t) || t == "("))
                {
                    op_stack.push("!".to_string());
                    prev_token = Some("!".to_string());
                    continue;
                }

                while let Some(top) = op_stack.last() {
                    if is_operator(top) {
                        if (is_right_associative(&token) && precedence(&token) < precedence(top))
                            || (!is_right_associative(&token)
                                && precedence(&token) <= precedence(top))
                        {
                            rpn_stack.push(RpnExpr::Basic(op_stack.pop().unwrap()));
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
                if let Ok(number) = token.parse::<i128>() {
                    rpn_stack.push(RpnExpr::Complex(Expr::Number(number)));
                } else if let Ok(number) = token.parse::<f64>() {
                    rpn_stack.push(RpnExpr::Complex(Expr::Float(number)));
                } else if token == "true" {
                    rpn_stack.push(RpnExpr::Complex(Expr::Bool(true)));
                } else if token == "false" {
                    rpn_stack.push(RpnExpr::Complex(Expr::Bool(false)));
                } else if token.starts_with('"') {
                    i += 1;
                    token.push(' ');
                    while i < primitives.len() {
                        token.push_str(primitives[i]);
                        if primitives[i].ends_with('"') {
                            break;
                        } else {
                            token.push(' ')
                        }
                        i += 1;
                    }
                    rpn_stack.push(RpnExpr::Complex(Expr::String(
                        token.trim_matches('"').to_string(),
                    )));
                } else
                /* if tkn.chars().all(char::is_alphanumeric) */
                if i < primitives.len() - 1 && primitives[i + 1] == "(" {
                    i += 2;
                    let mut params = vec![];

                    while i < primitives.len() && primitives[i] != ")" {
                        let arg_tokens = collect_expression_until(&primitives, &mut i, |t, _| {
                            t == "," || t == ")"
                        })
                        .unwrap();
                        let arg_expr = rpn_to_expr(rpn(&arg_tokens)).unwrap();
                        params.push(arg_expr);

                        if primitives[i] == "," {
                            i += 1;
                        }
                    }

                    if primitives[i] != ")" {
                        panic!("Unmatched parentheses in function call");
                    }
                    // i += 1;

                    rpn_stack.push(RpnExpr::Complex(Expr::FnCall {
                        name: token.clone(),
                        params,
                    }));
                } else {
                    rpn_stack.push(RpnExpr::Complex(Expr::Variable(token.clone())));
                }

                prev_token = Some(token);
            }
        }
        i += 1;
    }

    while let Some(op) = op_stack.pop() {
        rpn_stack.push(RpnExpr::Basic(op));
    }

    rpn_stack
}

pub fn rpn_to_expr(tokens: Vec<RpnExpr>) -> Result<Expr, String> {
    let mut stack: Vec<Expr> = Vec::new();

    if tokens.len() == 1 {
        if let RpnExpr::Basic(tkn) = tokens[0].clone() {
            if let Ok(value) = tkn.parse::<i128>() {
                return Ok(Expr::Number(value));
            } else if let Ok(value) = tkn.parse::<f64>() {
                return Ok(Expr::Float(value));
            } else if tkn.starts_with('"') && tkn.ends_with('"') {
                let trimmed = tkn.trim_matches('"').to_string(); //* Remove surrounding quotes
                return Ok(Expr::String(trimmed));
            } else if tkn.to_lowercase() == "true" {
                return Ok(Expr::Bool(true));
            } else if tkn.to_lowercase() == "false" {
                return Ok(Expr::Bool(false));
            } else {
                return Ok(Expr::Variable(tkn));
            }
        }
    }

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];

        match token {
            RpnExpr::Basic(tkn) => {
                match tkn.to_lowercase().as_str() {
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
                    }
                }
            }
            RpnExpr::Complex(c) => stack.push(c.clone()),
        }
        i += 1;
    }

    //* The final AST should be the only element left on the stack
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        println!("\n{:#?}\n", tokens);
        println!("\n{:#?}\n", stack);
        Err("Invalid postfix expression: leftover elements in stack".to_string())
    }
}

fn collect_expression_until<F>(
    tokens: &[&str],
    i: &mut usize,
    is_delimiter: F,
) -> Result<String, String>
where
    F: Fn(&str, usize) -> bool,
{
    let mut expression = String::new();
    let mut parentheses = 0;

    while *i < tokens.len() && (!is_delimiter(tokens[*i], *i) || parentheses > 0) {
        let token = &tokens[*i];

        match *token {
            "(" => parentheses += 1,
            ")" => {
                if parentheses == 0 {
                    return Err("Unmatched closing parenthesis".to_string());
                }
                parentheses -= 1;
            }
            _ => {}
        }

        if *token != "\n" {
            expression.push_str(token);
            expression.push(' ');
        }

        *i += 1;
    }

    if parentheses > 0 {
        return Err("Unmatched opening parenthesis".to_string());
    }

    Ok(expression.trim().to_string())
}
