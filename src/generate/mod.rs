use crate::ast::types::{Expr, Operator};

pub fn rust(ast: &[Expr]) -> String {
    format!("\n
fn main() {{
{}
}}

fn print(list: &[&Value]) {{
    for v in list {{
        print!(\"{{}} \", v.as_str())
    }}
    println!()
}}
", ast_to_rust(ast, 1))
}

pub fn ast_to_rust(ast: &[Expr], lvl: i128) -> String {
    ast
        .into_iter()
        .map(|expr| expr_to_rust(expr, lvl))
        .collect::<Vec<_>>()
        .join("")
}

fn expr_to_rust(node: &Expr, lvl: i128) -> String {
    match node {
        Expr::VarDec { name, value, line: _}  => format!(
            "{}let mut {} = {};\n",
            indent(lvl),
            name,
            expr_to_rust(value, lvl+1)
        ),
        Expr::Assign { name, value, line: _}  => format!(
            "{}{} = {};\n",
            indent(lvl),
            name,
            expr_to_rust(value, lvl+1)
        ),

        Expr::BinaryOp { left, op, right } => format!(
            "{}.{}(&{})",
            expr_to_rust(left, lvl),
            operation(op),
            expr_to_rust(right, lvl)
        ),

        Expr::If { condition, body } => format!(
            "\n{}if {}.as_bool() {{\n{}{}}}\n\n",
            indent(lvl),
            expr_to_rust(condition, lvl),
            ast_to_rust(body, lvl+1),
            indent(lvl)
        ),
        Expr::While { condition, body } => format!(
            "\n{}while {}.as_bool() {{\n{}{}}}\n\n",
            indent(lvl),
            expr_to_rust(condition, lvl),
            ast_to_rust(body, lvl+1),
            indent(lvl)
        ),

        Expr::FnCall { name, params } => format!(
            "{}{}(&vec![{}]);\n",
            indent(lvl),
            name,
            params
                .into_iter()
                .map(|param| format!("&{}", expr_to_rust(param, lvl)))
                .collect::<Vec<_>>()
                .join(", ")
            ),

        Expr::String(value) => format!("Value::String({:?}.to_string())", value),
        Expr::Not(value) => format!("{}.not()", expr_to_rust(value, lvl)),
        Expr::Number(value) => format!("Value::Number({})", value),
        Expr::Float(value) => format!("Value::Float({:?})", value),
        Expr::Bool(value) => format!("Value::Bool({})", value),
        Expr::Variable {name, line: _} => format!("{}", name),

        _ => "".to_string()
        
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
        Operator::Or => "or"
    }.to_string()
}

fn indent(lvl: i128) -> String {
    let mut indent = String::new();
    for _ in 0..lvl { indent.push_str("    ") }
    indent
}