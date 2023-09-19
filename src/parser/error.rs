use std::{error::Error, fmt::Display};

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    illegal_token: Token,
}

impl SyntaxError {
    pub fn new(t: Token) -> Self {
        Self { illegal_token: t }
    }
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Syntax error: the token {:?} isn't authorized here.",
            self.illegal_token
        )
    }
}
