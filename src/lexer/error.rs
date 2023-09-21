use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum LexerError {
    IllegalCharacterError(char),
    EmptyProgramError,
}

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalCharacterError(c) => write!(
                f,
                "Illegal Character Error: character '{}' is an illegal character",
                c
            ),
            Self::EmptyProgramError => {
                write!(f, "Empty Program Error : the provided program is empty")
            }
        }
    }
}
