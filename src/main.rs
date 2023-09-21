use std::{
    env,
    error::Error,
    fmt::Display,
    fs,
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::interpreter::execute;

mod ast;
mod interpreter;
mod lexer;
mod parser;

#[derive(Debug)]
struct CliError {}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: too manny parameters")
    }
}

fn interactive_mode() {
    println!("Welcome to the interactive mode of lambda calculus interpreter !");
    println!("Type \"quit\" or C-d to exit");
    let mut buf = String::new();
    loop {
        buf.clear();
        print!(">>> ");
        let _ = stdout().flush();
        if stdin().read_line(&mut buf).unwrap() == 0 || buf.trim() == "quit" {
            return;
        };
        let buf = buf.trim();
        match execute(buf) {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        };
    }
}

fn file_mode(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    match execute(contents.trim()) {
        Ok(result) => println!("{}", result),
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        interactive_mode();
    } else if args.len() == 2 {
        file_mode(&args[1]);
    } else {
        eprintln!("CliError: Too many Arguments");
        exit(1);
    }
}
