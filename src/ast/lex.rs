use regex::Regex;

use super::types::{Token, TokenKind};


pub fn lexer(source: &str) -> Vec<Token> {
    let pattern = r#"([a-zA-Z#_,@]+[a-zA-Z0-9#_,@]*|\d+\.\d+|\d+|[&"'+\-*!/=><();{}\s])"#;
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
        }else if is_keyword(&token){
            kind = TokenKind::Keyword;
        } else if token == "!" {
            kind = TokenKind::Not;
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

fn is_keyword(token: &str) -> bool {
    match &*token.to_lowercase() {
        "if" => { return true }
        "else" => { return true }

        "for" => { return true }
        "while" => { return true }

        "fn" => { return true }
        "return" => { return true }

        _ => { return false }
    }
}