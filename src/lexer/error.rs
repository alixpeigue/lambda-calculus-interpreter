use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct IllegalCharacterError {
    c: char,
}

impl IllegalCharacterError {
    pub fn new(c: char) -> Self {
        Self { c }
    }
}

impl Error for IllegalCharacterError {}

impl Display for IllegalCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Illegal Character Error: character '{}' is an illegal character",
            self.c
        )
    }
}
