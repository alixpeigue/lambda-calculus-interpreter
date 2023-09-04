use std::rc::Rc;

#[derive(PartialEq, Debug)]
enum Paren {
    Open,
    Close,
}

#[derive(PartialEq, Debug)]
enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Sup,
    Inf,
    SupEq,
    InfEq,
    Eq,
    Neq,
}

#[derive(PartialEq, Debug)]
enum Token {
    Lambda,
    Identifier(Rc<str>),
    Dot,
    Parentheses(Paren),
    BoolValue(bool),
    Numvalue(f64),
    If,
    Else,
    Operator(Op),
}

impl Token {
    fn identifier(name: &str) -> Self {
        Token::Identifier(Rc::from(name))
    }
}

fn lexer(prog: &str) -> Vec<Token> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn test_simple() {
        assert_eq!(
            lexer(r"\x.x"),
            vec![
                Token::Lambda,
                Token::identifier("x"),
                Token::Dot,
                Token::identifier("x")
            ]
        );

        assert_eq!(
            lexer(r"(\x.x+1) 1"),
            vec![
                Token::Parentheses(Paren::Open),
                Token::Lambda,
                Token::identifier("x"),
                Token::Dot,
                Token::identifier("x"),
                Token::Operator(Op::Plus),
                Token::Numvalue(1.),
                Token::Parentheses(Paren::Close),
                Token::Numvalue(1.)
            ]
        );
    }
}
