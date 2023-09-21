pub mod error;

use self::error::LexerError;

#[derive(PartialEq, Debug, Clone)]
pub enum Paren {
    Open,
    Close,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Sup,
    SupEq,
    Inf,
    InfEq,
    Eq,
    Neq,
    Not,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Lambda,
    Identifier(String),
    Dot,
    Parentheses(Paren),
    Colon,
    QuestionMark,
    Operator(Op),
}

impl Token {
    pub fn identifier(s: &str) -> Self {
        Self::Identifier(s.to_owned())
    }
}

pub fn lexer(prog: &str) -> Result<Vec<Token>, LexerError> {
    if prog.len() == 0 {
        return Err(LexerError::EmptyProgramError);
    }

    let mut last = 0;
    let mut res = vec![];
    for (index, matched) in prog.match_indices(|c: char| !c.is_alphanumeric()) {
        let sep = matched.chars().next().unwrap(); // should never panic
        let between = &prog[last..index];
        if (!between.chars().any(|c| !c.is_alphanumeric())) && between.len() > 0 {
            res.push(Token::identifier(between));
        }
        last = index + 1;
        let token = match sep {
            '\\' => Token::Lambda,
            '.' => Token::Dot,
            '(' => Token::Parentheses(Paren::Open),
            ')' => Token::Parentheses(Paren::Close),
            '?' => Token::QuestionMark,
            ':' => Token::Colon,
            '+' => Token::Operator(Op::Plus),
            '-' => Token::Operator(Op::Minus),
            '*' => Token::Operator(Op::Asterisk),
            '/' => Token::Operator(Op::Slash),
            '>' => Token::Operator(Op::Sup),
            '<' => Token::Operator(Op::Inf),
            '=' => match res.pop() {
                Some(el) => match el {
                    Token::Operator(Op::Sup) => Token::Operator(Op::SupEq),
                    Token::Operator(Op::Inf) => Token::Operator(Op::InfEq),
                    Token::Operator(Op::Not) => Token::Operator(Op::Neq),
                    other => {
                        res.push(other);
                        Token::Operator(Op::Eq)
                    }
                },
                None => Token::Operator(Op::Eq),
            },
            '!' => Token::Operator(Op::Not),
            ' ' => continue,
            other => return Err(LexerError::IllegalCharacterError(other)),
        };
        res.push(token);
    }
    let between = &prog[last..prog.len()];
    if (!between.chars().any(|c| !c.is_alphanumeric())) && between.len() > 0 {
        res.push(Token::Identifier(between.to_owned()));
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn test_simple() {
        assert_eq!(
            lexer(r"\x.x"),
            Ok(vec![
                Token::Lambda,
                Token::identifier("x"),
                Token::Dot,
                Token::identifier("x")
            ])
        );

        assert_eq!(
            lexer(r"(\x.x+1) 1"),
            Ok(vec![
                Token::Parentheses(Paren::Open),
                Token::Lambda,
                Token::identifier("x"),
                Token::Dot,
                Token::identifier("x"),
                Token::Operator(Op::Plus),
                Token::identifier("1"),
                Token::Parentheses(Paren::Close),
                Token::identifier("1"),
            ])
        );
    }

    #[test]
    fn test_two_chars_operators() {
        assert_eq!(lexer(r">="), Ok(vec![Token::Operator(Op::SupEq)]));
        assert_eq!(
            lexer(r"x!=y"),
            Ok(vec![
                Token::identifier("x"),
                Token::Operator(Op::Neq),
                Token::identifier("y")
            ])
        );
    }

    #[test]
    fn test_fib() {
        use Token::*;
        assert_eq!(
            lexer(r"\f.\x. x>2 ? f(x-1) + f(x-2) : 1"),
            Ok(vec![
                Lambda,
                Token::identifier("f"),
                Dot,
                Lambda,
                Token::identifier("x"),
                Dot,
                Token::identifier("x"),
                Operator(Op::Sup),
                Token::identifier("2"),
                QuestionMark,
                Token::identifier("f"),
                Parentheses(Paren::Open),
                Token::identifier("x"),
                Operator(Op::Minus),
                Token::identifier("1"),
                Parentheses(Paren::Close),
                Operator(Op::Plus),
                Token::identifier("f"),
                Parentheses(Paren::Open),
                Token::identifier("x"),
                Operator(Op::Minus),
                Token::identifier("2"),
                Parentheses(Paren::Close),
                Colon,
                Token::identifier("1")
            ])
        );
    }
}
