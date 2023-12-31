pub mod error;

use crate::{
    ast::{ArithmeticOp, ComparisonOp, Expr},
    interpreter::error::InterpreterError,
    lexer::lexer,
    parser::parse,
};

use std::{collections::HashMap, error::Error, fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum EvalResult {
    Value(f64),
    Boolean(bool),
    Closure {
        var: Rc<str>,
        body: Rc<Expr>,
        context: Env,
    },
}

impl Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalResult::Value(val) => write!(f, "{}", val),
            EvalResult::Boolean(val) => write!(f, "{}", val),
            EvalResult::Closure { .. } => write!(f, "Closure"),
        }
    }
}

impl ArithmeticOp {
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        match self {
            ArithmeticOp::Add => x + y,
            ArithmeticOp::Sub => x - y,
            ArithmeticOp::Mul => x * y,
            ArithmeticOp::Div => x / y,
        }
    }
}

impl ComparisonOp {
    pub fn eval(&self, x: f64, y: f64) -> bool {
        match self {
            ComparisonOp::Gt => x > y,
            ComparisonOp::Gte => x >= y,
            ComparisonOp::Lt => x < y,
            ComparisonOp::Lte => x <= y,
            ComparisonOp::Eq => x == y,
            ComparisonOp::Neq => x != y,
        }
    }
}

type Env = HashMap<Rc<str>, EvalResult>;

impl Expr {
    pub fn eval(&self) -> Result<EvalResult, InterpreterError> {
        self.eval_rec(&HashMap::new())
    }

    fn eval_rec(&self, env: &Env) -> Result<EvalResult, InterpreterError> {
        match self {
            Expr::Abs { var, body } => Ok(EvalResult::Closure {
                body: Rc::clone(body),
                var: var.clone(),
                context: env.clone(),
            }),
            Expr::App {
                function,
                parameter,
            } => {
                let parameter = parameter.eval_rec(env)?;
                let function = function.eval_rec(env)?;
                match function {
                    EvalResult::Closure {
                        var,
                        body,
                        mut context,
                    } => {
                        context.insert(var, parameter);
                        body.eval_rec(&context)
                    }
                    other => Err(InterpreterError::new_type_error(
                        "Closure",
                        &format!("{:?}", other),
                    )),
                }
            }
            Expr::Arithmetic {
                operation,
                lhs,
                rhs,
            } => {
                let lhs = lhs.eval_rec(env)?;
                let rhs = rhs.eval_rec(env)?;
                match (lhs, rhs) {
                    (EvalResult::Value(lhs), EvalResult::Value(rhs)) => {
                        Ok(EvalResult::Value(operation.eval(lhs, rhs)))
                    }
                    other => Err(InterpreterError::new_type_error(
                        "Value",
                        &format!("{:?}", other),
                    )),
                }
            }
            Expr::Var { name } => env
                .get(name)
                .cloned()
                .ok_or(InterpreterError::new_name_error(name)),
            Expr::NumericLiteral { value } => Ok(EvalResult::Value(*value)),
            Expr::Conditional {
                condition,
                true_branch,
                false_branch,
            } => match condition.eval_rec(env)? {
                EvalResult::Boolean(cond) => {
                    if cond {
                        true_branch.eval_rec(env)
                    } else {
                        false_branch.eval_rec(env)
                    }
                }
                other => Err(InterpreterError::new_type_error(
                    "Boolean",
                    &format!("{:?}", other),
                )),
            },
            Expr::Comparison {
                operation,
                lhs,
                rhs,
            } => match (lhs.eval_rec(env)?, rhs.eval_rec(env)?) {
                (EvalResult::Value(lhs), EvalResult::Value(rhs)) => {
                    Ok(EvalResult::Boolean(operation.eval(lhs, rhs)))
                }
                (EvalResult::Value(_), other) | (other, _) => {
                    // dbg!(env);
                    Err(InterpreterError::new_type_error(
                        "Value",
                        &format!("{:?}", other),
                    ))
                }
            },
            Expr::BooleanLiteral { value } => Ok(EvalResult::Boolean(*value)),
        }
    }
}

pub fn execute(program: &str) -> Result<EvalResult, Box<dyn Error>> {
    Ok(parse(&lexer(program)?)?.eval()?)
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::interpreter::*;

    #[test]
    fn test_simple_addition_lamda() {
        let ast = Expr::app(
            Expr::abs(
                "x",
                Expr::arithmetic(ArithmeticOp::Add, Expr::var("x"), Expr::numeric_literal(1.)),
            ),
            Expr::numeric_literal(1.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(2.)));
    }

    #[test]
    fn test_simple_comparison() {
        let ast = Expr::comparison(
            ComparisonOp::Gt,
            Expr::numeric_literal(2.),
            Expr::numeric_literal(1.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Boolean(true)))
    }

    #[test]
    fn test_simple_conditional() {
        let ast = Expr::conditional(
            Expr::boolean_literal(true),
            Expr::numeric_literal(1.),
            Expr::numeric_literal(2.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(1.)))
    }

    #[test]
    fn test_conditional_comparison() {
        let ast = Expr::conditional(
            Expr::comparison(
                ComparisonOp::Gt,
                Expr::numeric_literal(1.),
                Expr::numeric_literal(2.),
            ),
            Expr::numeric_literal(1.),
            Expr::numeric_literal(2.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(2.)));
    }

    #[test]
    fn complex_case_ast() {
        let ast = Expr::app(
            Expr::app(
                Expr::abs(
                    "x",
                    Expr::abs("y", Expr::app(Expr::var("y"), Expr::var("x"))),
                ),
                Expr::numeric_literal(1.),
            ),
            Expr::app(
                Expr::abs(
                    "x",
                    Expr::abs(
                        "y",
                        Expr::arithmetic(ArithmeticOp::Add, Expr::var("x"), Expr::var("y")),
                    ),
                ),
                Expr::numeric_literal(2.),
            ),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(3.)));
    }

    #[test]
    fn test_fib() {
        let inner = Expr::abs(
            "x",
            Expr::app(
                Expr::var("f"),
                Expr::abs(
                    "v",
                    Expr::app(Expr::app(Expr::var("x"), Expr::var("x")), Expr::var("v")),
                ),
            ),
        );
        let y_comb = Expr::abs("f", Expr::app(inner.clone(), inner));

        let fib_norec = Expr::abs(
            "f",
            Expr::abs(
                "x",
                Expr::conditional(
                    Expr::comparison(ComparisonOp::Lt, Expr::var("x"), Expr::numeric_literal(2.)),
                    Expr::numeric_literal(1.),
                    Expr::arithmetic(
                        ArithmeticOp::Add,
                        Expr::app(
                            Expr::var("f"),
                            Expr::arithmetic(
                                ArithmeticOp::Sub,
                                Expr::var("x"),
                                Expr::numeric_literal(1.),
                            ),
                        ),
                        Expr::app(
                            Expr::var("f"),
                            Expr::arithmetic(
                                ArithmeticOp::Sub,
                                Expr::var("x"),
                                Expr::numeric_literal(2.),
                            ),
                        ),
                    ),
                ),
            ),
        );

        let fib = Expr::app(y_comb, fib_norec);

        let fib_5 = Expr::app(fib, Expr::numeric_literal(5.));

        assert_eq!(fib_5.eval(), Ok(EvalResult::Value(8.)));
    }
}
