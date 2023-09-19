use std::io::{self, stdin};

use lexer::lexer;
use parser::parse;

mod ast;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let tokens = lexer(buf.trim()).unwrap();
    let ast = parse(&tokens).unwrap();
    println!("{:?}", ast.eval().unwrap());
}
