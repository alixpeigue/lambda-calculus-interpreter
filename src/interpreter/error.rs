use std::{error::Error, fmt::Display};

use super::EvalResult;

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    TypeError {
        wanted_type: String,
        given_type: String,
    },
    NameError {
        name: String,
    },
}

impl InterpreterError {
    pub fn new_type_error(wanted_type: &str, given_type: &str) -> Self {
        Self::TypeError {
            wanted_type: wanted_type.to_owned(),
            given_type: given_type.to_owned(),
        }
    }
    pub fn new_name_error(name: &str) -> Self {
        Self::NameError {
            name: name.to_owned(),
        }
    }
}

impl Error for InterpreterError {}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::TypeError {
                wanted_type,
                given_type,
            } => write!(
                f,
                "TypeError: type needed : {}, bot got type {}",
                wanted_type, given_type
            ),
            InterpreterError::NameError { name } => write!(
                f,
                "Unknown name {:?} : this name cannot be bound to a value because it is unknown",
                name
            ),
        }
    }
}
