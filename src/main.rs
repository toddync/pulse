use clap::Parser;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

mod ast;
mod generate;
mod types;

use ast::create;
use types::VALUE;

/// "compiler" for the pulse language
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to the input file
    #[arg(short, long)]
    file: String,

    /// path to the output file
    #[arg(short, long, default_value_t = String::from("-"))]
    output: String,

    /// whether the output code should be in rust
    #[arg(short, long, default_value_t = true)]
    rust: bool,
}

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let args = Args::parse();
    let contents = fs::read_to_string(args.file).expect("\nfile not found\n");
    let ast = create(&contents);
    let code = generate::rust(&ast);
    fs::write(&args.output, format!("{}{}", VALUE, code)).unwrap();
    println!("Code saved to {}", args.output);

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start;
    println!("\n{:#?}\n", ast);
    println!("time: {:?}", time);
}
