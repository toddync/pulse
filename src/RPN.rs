fn rpn(expression: &str) -> Vec<String> {
    let mut rpn: Vec<String> = Vec::new(); // Resulting RPN expression.
    let mut op_stack: Vec<String> = Vec::new(); // Stack to hold operators.

    // Helper functions for operator properties.
    fn is_operator(token: &str) -> bool {
        matches!(token, "*" | "/" | "+" | "-" | "^" | "&&" | "||" | "and" | "or")
    }

    fn is_right_associative(op: &str) -> bool {
        op == "^"
    }

    fn precedence(op: &str) -> i32 {
        match op {
            "^" => 4,
            "*" | "/" => 3,
            "+" | "-" => 2,
            "&&" | "and" => 1,
            "||" | "or" => 0,
            _ => -1,
        }
    }

    // Tokenize the expression.
    for token in expression.split_whitespace() {
        match token {
            "(" => op_stack.push(token.to_string()),
            ")" => {
                // Pop operators until "(" is found.
                while let Some(top) = op_stack.last() {
                    if top == "(" {
                        break;
                    }
                    rpn.push(op_stack.pop().unwrap());
                }
                op_stack.pop(); // Remove the "(".
            }
            _ if is_operator(token) => {
                // Handle operator precedence and associativity.
                while let Some(top) = op_stack.last() {
                    if is_operator(top) {
                        if (is_right_associative(token) && precedence(token) < precedence(top)) || (!is_right_associative(token) && precedence(token) <= precedence(top)) {
                            rpn.push(op_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                op_stack.push(token.to_string());
            }
            _ => {
                // Operand (number, variable, or boolean).
                rpn.push(token.to_string());
            }
        }
    }

    // Pop remaining operators onto the RPN stack.
    while let Some(op) = op_stack.pop() {
        rpn.push(op);
    }

    rpn
}
