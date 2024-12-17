use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    //* variables, reserved words and chars
    Identifier,
    Semicolon,
    Delimiter,
    Keyword,
    Comma,

    //* literals
    String,
    Number,
    Float,
    Bool,

    //* math
    Operator,

    //* assignment
    OpAssign,
    Assign,

    //* logic
    NotEqual,
    Greater,
    GtEqual,
    Smaller,
    SmEqual,
    Equal,
    Not,
    And,
    Or,

    //* others
    EOF,
    Nl,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Variable(String),
    String(String),
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
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Subtract,
    Multiply,
    Modulus,
    Divide,
    Power,
    Add
}

fn main() {
    let contents = fs::read_to_string("./source/test.nv")
        .expect("Should have been able to read the file");

    let tokens = lexer(&contents);
    let ast = ast(&tokens);

    println!("{:#?}", ast);
}

fn ast(tokens: &Vec<Token>) -> Vec<Expr> {
    let mut instructions: Vec<Expr> = vec![];

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        if token.kind == TokenKind::Identifier {
            if tokens[i+1].kind == TokenKind::Assign {
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
                    } else if piece.kind == TokenKind::EOF {
                        panic!("unexpected end of file during assignment parsing")
                    }

                    expression.push_str(&piece.value);
                    expression.push(' ');

                    if piece.kind != TokenKind::Nl {
                        last_tkn = piece;
                    }
                    i += 1;
                }

                let value = parse_postfix_to_ast(rpn(&expression)).unwrap();
                instructions.push(Expr::VarDec { name: token.value.clone(), value: Box::new(value) });
            }
        }
        i += 1;
    }

    instructions
}

fn lexer(source: &str) -> Vec<Token> {
    let pattern = r#"([a-zA-Z#_,@!]+[a-zA-Z0-9#_,@!]*|\d+\.\d+|\d+|[&"'+\-*!/=><();{}\s])"#;
    let regex = Regex::new(pattern).expect("Failed to compile regex");

    let primitives: Vec<&str> = regex
        .find_iter(source)
        .map(|m| m.as_str())
        .collect();

    let mut tokens: Vec<Token> = vec![];

    let operators = "^/+*-%".to_string();
    let delimiters = "{}[]()".to_string();
    
    let mut i = 0;
    while  i < primitives.len() {
        let mut token = primitives[i].to_string();
        let mut kind = TokenKind::Identifier;
        
        if operators.contains(&token) {
            if primitives[i+1] == "=" {
                token.push_str("=");
                kind = TokenKind::OpAssign;
                i += 1;
            } else {
                kind = TokenKind::Operator;
            }
        } else if delimiters.contains(&token) {
            kind = TokenKind::Delimiter;
        } else if token == "\"" {
            kind = TokenKind::String;
            i += 1;
            while  i < primitives.len() && primitives[i] != "\"" {
                token.push_str(primitives[i]);
                i += 1;
            }
            token.push_str(primitives[i]);
        } else if let Ok(_) = token.parse::<i64>() {
            kind = TokenKind::Number
        } else if let Ok(_) = token.parse::<f64>() {
            kind = TokenKind::Float
        } else if token.to_lowercase() == "true" || token.to_lowercase() == "false" {
            kind = TokenKind::Bool;
        } else if token.to_lowercase() == "and" || token == "&" {
            kind = TokenKind::And;
        } else if token.to_lowercase() == "or" || token == "|" {
            kind = TokenKind::Or;
        } else if token == "," {
            kind = TokenKind::Comma;
        } else if token == "=" {
            if primitives[i+1] == "=" {
                token.push_str(primitives[i+1]);
                kind = TokenKind::Equal;
                i += 1;
            } else {
                kind = TokenKind::Assign;
            }
        } else if token == ";" {
            kind = TokenKind::Semicolon;
        } else if token == "\n" {
            kind = TokenKind::Nl;
        }
        
        if token != " " {
            tokens.push(Token {
                value: token,
                kind: kind,
            });
        }
        i+= 1;
    }

    tokens.push(Token {
        value: "EOF".to_string(),
        kind: TokenKind::EOF,
    });

    tokens
}

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


fn is_literal(kind: &TokenKind) -> bool {
    if  kind == &TokenKind::String ||
        kind == &TokenKind::Number ||
        kind == &TokenKind::Float  ||
        kind == &TokenKind::Bool
    {
        return true;
    } else {
        return false;
    }
}

pub fn parse_postfix_to_ast(tokens: Vec<String>) -> Result<Expr, String> {
    let mut stack: Vec<Expr> = Vec::new();

    if tokens.len() == 1 {
        let token = tokens.into_iter().next().unwrap(); // Take ownership of the only token
        if let Ok(value) = token.parse::<i64>() {
            return Ok(Expr::Number(value))
        } else if let Ok(value) = token.parse::<f64>() {
            return Ok(Expr::Float(value))
        } else if token.starts_with('"') && token.ends_with('"') {
            // Check for proper surrounding quotes for a string
            let trimmed = token.trim_matches('"').to_string(); // Remove surrounding quotes
            return Ok(Expr::String(trimmed))
        } else if token.to_lowercase() == "true" {
            return Ok(Expr::Bool(true))
        } else if  token.to_lowercase() == "false" {
            return Ok(Expr::Bool(false))
        } else {
            return Ok(Expr::Variable(token))
        }
    }


    for token in tokens {
        match token.as_str() {
            "+" => {
                let right = stack.pop().ok_or("Stack underflow for '+' operator")?;
                let left = stack.pop().ok_or("Stack underflow for '+' operator")?;
                stack.push(Expr::BinaryOp {
                    left: Box::new(left),
                    op: Operator::Add,
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
            _ => {
                // Handle operands: try parsing as a number or assume it's a variable
                if let Ok(number) = token.parse::<i64>() {
                    stack.push(Expr::Number(number));
                }else if let Ok(number) = token.parse::<f64>() {
                    stack.push(Expr::Float(number));
                } else if token == "true" {
                    stack.push(Expr::Bool(true))
                } else if  token == "false" {
                    stack.push(Expr::Bool(false))
                } else if token.chars().all(char::is_alphanumeric) {
                    stack.push(Expr::Variable(token));
                } else {
                    return Err(format!("Invalid token: {}", token));
                }
            }
        }
    }

    // The final AST should be the only element left on the stack
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("Invalid postfix expression: leftover elements in stack".to_string())
    }
}