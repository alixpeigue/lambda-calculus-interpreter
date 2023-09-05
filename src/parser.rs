use super::ast::Expr;
use super::lexer::{Op, Paren, Token};

fn parse(tokens: &[Token]) -> Result<Expr, String> {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    fn test_simple() {
        assert_eq!(
            parse(&vec![
                Token::Lambda,
                Token::Identifier("x"),
                Token::Dot,
                Token::Identifier("x")
            ]),
            Ok(Expr::Abs("x", Expr::Var("x")))
        );
        assert_eq!(
            parse(&vec![
                Token::Parentheses(Paren::Open),
                Token::Lambda,
                Token::Identifier("x"),
                Token::Dot,
                Token::Identifier("x"),
                Token::Parentheses(Paren::Close),
                Token::Identifier("1")
            ]),
            Ok(Expr::App(
                Expr::Abs("x", Expr::Var("x")),
                Expr::NumericLiteral(1.)
            ))
        );
    }
}
