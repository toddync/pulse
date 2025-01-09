use crate::ast::types::{Expr, Operator};

pub fn rust(ast: &[Expr]) -> String {
    format!(
        "\n
fn main() {{
{}}}

fn print(list: &[&Value]) -> Value {{
    for v in list {{ print!(\"{{}} \", v.as_str()) }}
    println!();
    Value::Undefined
}}

fn add(list: &Vec<&Value>) -> Value {{
    let mut a = list[0].clone();
    let mut i = 1;
    while i < list.len() {{ a = a.add(list[i]); i += 1 }}
    a
}}
",
        ast_to_rust(ast, 1)
    )
}

pub fn ast_to_rust(ast: &[Expr], lvl: i128) -> String {
    ast.iter()
        .map(|expr| expr_to_rust(expr, lvl))
        .collect::<Vec<_>>()
        .join("")
}

fn expr_to_rust(node: &Expr, lvl: i128) -> String {
    match node {
        Expr::VarDec {
            name,
            value,
            line: _,
        } => format!(
            "{}let mut {} = {};\n",
            indent(lvl),
            name,
            expr_to_rust_inline(value)
        ),
        Expr::Assign {
            name,
            value,
            line: _,
        } => format!(
            "{}{} = {};\n",
            indent(lvl),
            name,
            expr_to_rust_inline(value)
        ),

        Expr::If { condition, body } => format!(
            "\n{}if {}.as_bool() {{\n{}{}}}\n\n",
            indent(lvl),
            expr_to_rust_inline(condition),
            ast_to_rust(body, lvl + 1),
            indent(lvl)
        ),
        Expr::While { condition, body } => format!(
            "{}while {}.as_bool() {{\n{}{}}}\n",
            indent(lvl),
            expr_to_rust_inline(condition),
            ast_to_rust(body, lvl + 1),
            indent(lvl)
        ),

        Expr::FnCall { name, params } => format!(
            "{}{}(&vec![{}]);\n",
            indent(lvl),
            name,
            params
                .iter()
                .map(|param| format!("&{}", expr_to_rust_inline(param)))
                .collect::<Vec<_>>()
                .join(", ")
        ),

        Expr::String(value) => format!("Value::String({:?}.to_string())", value),
        Expr::Not(value) => format!("{}.not()", expr_to_rust(value, lvl)),
        Expr::Number(value) => format!("Value::Number({})", value),
        Expr::Float(value) => format!("Value::Float({:?})", value),
        Expr::Bool(value) => format!("Value::Bool({})", value),
        Expr::Variable(name) => name.to_string(),

        _ => String::new(),
    }
}

fn expr_to_rust_inline(node: &Expr) -> String {
    match node {
        Expr::BinaryOp { left, op, right } => format!(
            "{}.{}(&{})",
            expr_to_rust_inline(left),
            operation(op),
            expr_to_rust_inline(right)
        ),

        Expr::FnCall { name, params } => format!(
            "{}(&vec![{}])",
            name,
            params
                .iter()
                .map(|param| format!("&{}", expr_to_rust_inline(param)))
                .collect::<Vec<_>>()
                .join(", ")
        ),

        Expr::String(value) => format!("Value::String({:?}.to_string())", value),
        Expr::Not(value) => format!("{}.not()", expr_to_rust_inline(value)),
        Expr::Number(value) => format!("Value::Number({})", value),
        Expr::Float(value) => format!("Value::Float({:?})", value),
        Expr::Bool(value) => format!("Value::Bool({})", value),
        Expr::Variable(name) => name.to_string(),

        _ => String::new(),
    }
}
fn operation(op: &Operator) -> String {
    match op {
        Operator::Subtract => "sub",
        Operator::Multiply => "mul",
        Operator::Modulus => "modu",
        Operator::Divide => "div",
        Operator::Power => "pow",
        Operator::Add => "add",

        Operator::Different => "dif",
        Operator::LtEquals => "lte",
        Operator::GtEquals => "gte",
        Operator::Greater => "gt",
        Operator::Equals => "eq",
        Operator::Less => "lt",
        Operator::And => "and",
        Operator::Or => "or",
    }
    .to_string()
}

fn indent(lvl: i128) -> String {
    let mut indent = String::new();
    for _ in 0..lvl {
        indent.push_str("    ")
    }
    indent
}
