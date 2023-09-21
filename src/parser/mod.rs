pub mod error;

use crate::{
    ast::{ArithmeticOp, ComparisonOp, Expr},
    lexer::{Op, Paren, Token},
    parser::error::SyntaxError,
};

fn op_sup(op1: &Op, op2: &Op) -> bool {
    (*op1 == Op::Asterisk || *op1 == Op::Slash) && (*op2 == Op::Plus || *op2 == Op::Minus)
}

pub fn parse(tokens: &[Token]) -> Result<Expr, SyntaxError> {
    // dbg!(tokens);
    let tokens = remove_extra_parentheses(tokens);

    // Trying to match Abs:
    if let [Token::Lambda, Token::Identifier(id), Token::Dot, rest @ ..] = tokens {
        return Ok(Expr::abs(id, parse(rest)?));
    }

    // Trying to match Conditional
    let mut depth = 0;
    for i in 0..tokens.len() {
        if tokens[i] == Token::Parentheses(Paren::Close) {
            depth += 1;
        } else if tokens[i] == Token::Parentheses(Paren::Open) {
            depth -= 1;
        }
        if tokens[i] == Token::QuestionMark && depth == 0 {
            let mut depth = 0;
            for j in (i + 1..tokens.len()).rev() {
                if tokens[j] == Token::Parentheses(Paren::Close) {
                    depth += 1;
                } else if tokens[j] == Token::Parentheses(Paren::Open) {
                    depth -= 1;
                }
                if tokens[j] == Token::Colon && depth == 0 {
                    return Ok(Expr::conditional(
                        parse(&tokens[0..i])?,
                        parse(&tokens[i + 1..j])?,
                        parse(&tokens[j + 1..tokens.len()])?,
                    ));
                }
            }
        }
    }

    // Trying to match App:
    let mut depth = 0;
    for i in (1..tokens.len()).rev() {
        if tokens[i] == Token::Parentheses(Paren::Close) {
            depth += 1;
        } else if tokens[i] == Token::Parentheses(Paren::Open) {
            depth -= 1;
        }

        if depth == 0 {
            // We match try to find a non parenthesized Application
            // Application is either Identifier then Anything Except operator
            // or Closing paren then anything except operator
            match tokens[i - 1] {
                Token::Parentheses(Paren::Close) | Token::Identifier(_) => match tokens[i] {
                    Token::Operator(_) | Token::QuestionMark | Token::Colon | Token::Dot => {
                        continue
                    }
                    _ => {
                        return Ok(Expr::app(
                            parse(&tokens[0..i])?,
                            parse(&tokens[i..tokens.len()])?,
                        ))
                    }
                },
                _ => continue,
            }
        }
    }

    // It is not an abstraction, nor a conditional nor an application

    // Trying to match Arithmetic and Comparison

    let mut depth = 0;
    let mut index = 0;
    for (i, ref token) in tokens.iter().enumerate() {
        if **token == Token::Parentheses(Paren::Open) {
            depth += 1;
        } else if **token == Token::Parentheses(Paren::Close) {
            depth -= 1;
        } else if let Token::Operator(op) = token {
            if index != 0 {
                if let Token::Operator(ref op2) = tokens[index] {
                    if depth == 0 && !op_sup(op, op2) {
                        index = i;
                    }
                }
            } else if depth == 0 {
                index = i;
            }
        }
    }

    if index != 0 {
        use Token::*;
        return match tokens[index] {
            Operator(Op::Plus) => Ok(Expr::arithmetic(
                ArithmeticOp::Add,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::Minus) => Ok(Expr::arithmetic(
                ArithmeticOp::Sub,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::Asterisk) => Ok(Expr::arithmetic(
                ArithmeticOp::Mul,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::Slash) => Ok(Expr::arithmetic(
                ArithmeticOp::Div,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::Sup) => Ok(Expr::comparison(
                ComparisonOp::Gt,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::SupEq) => Ok(Expr::comparison(
                ComparisonOp::Gte,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::Inf) => Ok(Expr::comparison(
                ComparisonOp::Lt,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::InfEq) => Ok(Expr::comparison(
                ComparisonOp::Lte,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::Eq) => Ok(Expr::comparison(
                ComparisonOp::Eq,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            Operator(Op::Neq) => Ok(Expr::comparison(
                ComparisonOp::Neq,
                parse(&tokens[0..index])?,
                parse(&tokens[index + 1..])?,
            )),
            _ => panic!("Should never happen: {:?}", tokens[index]),
        };
    }

    // Mtching variables names and literal values

    match tokens {
        [Token::Identifier(first), Token::Dot, Token::Identifier(second)] => {
            match format!("{first}.{second}").parse::<f64>() {
                Ok(n) => Ok(Expr::numeric_literal(n)),
                _ => Err(SyntaxError::new(tokens[0].clone())),
            }
        }
        [Token::Identifier(id), Token::Dot] => match id.parse::<f64>() {
            Ok(n) => Ok(Expr::numeric_literal(n)),
            _ => Err(SyntaxError::new(tokens[0].clone())),
        },
        [Token::Identifier(id)] if *id == "true" => Ok(Expr::boolean_literal(true)),
        [Token::Identifier(id)] if *id == "false" => Ok(Expr::boolean_literal(false)),
        [Token::Identifier(id)] if id.starts_with(|c: char| c.is_alphabetic()) => Ok(Expr::var(id)),
        [Token::Identifier(id)] => {
            if let Ok(n) = id.parse::<f64>() {
                Ok(Expr::numeric_literal(n))
            } else {
                Err(SyntaxError::new(tokens[0].clone()))
            }
        }
        _ => Err(SyntaxError::new(tokens[0].clone())),
    }

    // Ok(Expr::Var("a"))
}

fn remove_extra_parentheses(tokens: &[Token]) -> &[Token] {
    if tokens.len() == 0 {
        return tokens;
    }
    if *tokens.iter().next().unwrap() == Token::Parentheses(Paren::Open)
        && *tokens.iter().last().unwrap() == Token::Parentheses(Paren::Close)
    {
        let mut depth = 0;
        for ref token in &tokens[..tokens.len() - 1] {
            if **token == Token::Parentheses(Paren::Open) {
                depth += 1;
            } else if **token == Token::Parentheses(Paren::Close) {
                depth -= 1;
            }
            if depth == 0 {
                return tokens;
            }
        }
        return remove_extra_parentheses(&tokens[1..tokens.len() - 1]);
    } else {
        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    #[test]
    fn test_simple() {
        assert_eq!(
            parse(&vec![
                Token::Lambda,
                Token::identifier("x"),
                Token::Dot,
                Token::identifier("x")
            ]),
            Ok(Expr::abs("x", Expr::var("x")))
        );
        assert_eq!(
            parse(&vec![
                Token::Parentheses(Paren::Open),
                Token::Lambda,
                Token::identifier("x"),
                Token::Dot,
                Token::identifier("x"),
                Token::Parentheses(Paren::Close),
                Token::identifier("1")
            ]),
            Ok(Expr::app(
                Expr::abs("x", Expr::var("x")),
                Expr::numeric_literal(1.)
            ))
        );
    }
}
