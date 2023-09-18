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
    println!(
        "{:?}",
        parse(&lexer(buf.trim()).unwrap()).unwrap().eval().unwrap()
    );
}
