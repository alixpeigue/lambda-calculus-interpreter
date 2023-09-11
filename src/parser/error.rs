use std::{error::Error, fmt::Display};

use crate::lexer::Token;

#[derive(Debug)]
struct SyntaxError<'a> {
    illegal_token: Token<'a>,
}

impl<'a> SyntaxError<'a> {
    fn new(t: Token<'a>) -> Self {
        Self { illegal_token: t }
    }
}

impl<'a> Error for SyntaxError<'a> {}

impl<'a> Display for SyntaxError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Syntax error: the token {:?} isn't authorized here.",
            self.illegal_token
        )
    }
}
