use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Abs {
        var: String,
        body: Box<Expr>,
    },
    App {
        function: Box<Expr>,
        parameter: Box<Expr>,
    },
    Arithmetic {
        operation: ArithmeticOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Comparison {
        operation: ComparisonOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Conditional {
        condition: Box<Expr>,
        true_branch: Box<Expr>,
        false_branch: Box<Expr>,
    },
    Var {
        name: String,
    },
    NumericLiteral {
        value: f64,
    },
    BooleanLiteral {
        value: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
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

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
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

#[derive(Debug, Clone, PartialEq)]
pub enum EvalResult {
    Value(f64),
    Boolean(bool),
    Closure {
        var: String,
        body: Expr,
        context: Env,
    },
}

type Env = HashMap<String, EvalResult>;

impl Expr {
    pub fn Abs(var: &str, body: Expr) -> Self {
        Expr::Abs {
            var: String::from(var),
            body: Box::new(body),
        }
    }

    pub fn App(function: Expr, parameter: Expr) -> Self {
        Expr::App {
            function: Box::new(function),
            parameter: Box::new(parameter),
        }
    }

    pub fn Arithmetic(operation: ArithmeticOp, lhs: Expr, rhs: Expr) -> Self {
        Expr::Arithmetic {
            operation,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn Comparison(operation: ComparisonOp, lhs: Expr, rhs: Expr) -> Self {
        Expr::Comparison {
            operation,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn Conditional(condition: Expr, true_branch: Expr, false_branch: Expr) -> Self {
        Expr::Conditional {
            condition: Box::new(condition),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        }
    }

    pub fn Var(name: &str) -> Self {
        Expr::Var {
            name: String::from(name),
        }
    }

    pub fn NumericLiteral(value: f64) -> Self {
        Expr::NumericLiteral { value }
    }

    pub fn BooleanLiteral(value: bool) -> Self {
        Expr::BooleanLiteral { value }
    }

    pub fn eval(&self) -> Result<EvalResult, String> {
        self.eval_rec(HashMap::new())
    }

    fn eval_rec(&self, env: Env) -> Result<EvalResult, String> {
        dbg!(&env);
        match self {
            Expr::Abs { var, body } => Ok(EvalResult::Closure {
                body: *body.clone(),
                var: var.clone(),
                context: env,
            }),
            Expr::App {
                function,
                parameter,
            } => {
                let parameter = parameter.eval_rec(env.clone())?;
                let function = function.eval_rec(env)?;
                match function {
                    EvalResult::Closure { var, body, context } => {
                        let mut context = context.clone();
                        context.insert(var, parameter);
                        body.eval_rec(context)
                    }
                    _ => Err("Could not coalesce to closure".to_string()),
                }
            }
            Expr::Arithmetic {
                operation,
                lhs,
                rhs,
            } => {
                let lhs = lhs.eval_rec(env.clone())?;
                let rhs = rhs.eval_rec(env)?;
                match (lhs, rhs) {
                    (EvalResult::Value(lhs), EvalResult::Value(rhs)) => {
                        Ok(EvalResult::Value(operation.eval(lhs, rhs)))
                    }
                    _ => Err("Could not coalesce expression to value".to_string()),
                }
            }
            Expr::Var { name } => env
                .get(name)
                .cloned()
                .ok_or(format!("Variable {name} cannot be bound")),
            Expr::NumericLiteral { value } => Ok(EvalResult::Value(*value)),
            Expr::Conditional {
                condition,
                true_branch,
                false_branch,
            } => match condition.eval_rec(env)? {
                EvalResult::Boolean(cond) => {
                    if cond {
                        true_branch.eval()
                    } else {
                        false_branch.eval()
                    }
                }
                _ => Err(String::from(
                    "Could not coalesce expression to boolean value",
                )),
            },
            Expr::Comparison {
                operation,
                lhs,
                rhs,
            } => match (lhs.eval_rec(env.clone())?, rhs.eval_rec(env)?) {
                (EvalResult::Value(lhs), EvalResult::Value(rhs)) => {
                    Ok(EvalResult::Boolean(operation.eval(lhs, rhs)))
                }
                _ => Err("Could not coalesce expression to boolean".to_string()),
            },
            Expr::BooleanLiteral { value } => Ok(EvalResult::Boolean(*value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_simple_addition_lamda() {
        let ast = Expr::App(
            Expr::Abs(
                "x",
                Expr::Arithmetic(ArithmeticOp::Add, Expr::Var("x"), Expr::NumericLiteral(1.)),
            ),
            Expr::NumericLiteral(1.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(2.)));
    }

    #[test]
    fn test_simple_comparison() {
        let ast = Expr::Comparison(
            ComparisonOp::Gt,
            Expr::NumericLiteral(2.),
            Expr::NumericLiteral(1.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Boolean(true)))
    }

    #[test]
    fn test_simple_conditional() {
        let ast = Expr::Conditional(
            Expr::BooleanLiteral(true),
            Expr::NumericLiteral(1.),
            Expr::NumericLiteral(2.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(1.)))
    }

    #[test]
    fn test_conditional_comparison() {
        let ast = Expr::Conditional(
            Expr::Comparison(
                ComparisonOp::Gt,
                Expr::NumericLiteral(1.),
                Expr::NumericLiteral(2.),
            ),
            Expr::NumericLiteral(1.),
            Expr::NumericLiteral(2.),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(2.)));
    }

    #[test]
    fn complex_case_ast() {
        let ast = Expr::App(
            Expr::App(
                Expr::Abs(
                    "x",
                    Expr::Abs("y", Expr::App(Expr::Var("y"), Expr::Var("x"))),
                ),
                Expr::NumericLiteral(1.),
            ),
            Expr::App(
                Expr::Abs(
                    "x",
                    Expr::Abs(
                        "y",
                        Expr::Arithmetic(ArithmeticOp::Add, Expr::Var("x"), Expr::Var("y")),
                    ),
                ),
                Expr::NumericLiteral(2.),
            ),
        );
        assert_eq!(ast.eval(), Ok(EvalResult::Value(3.)));
    }

    #[test]
    fn test_fib() {
        let inner = Expr::Abs(
            "x",
            Expr::App(
                Expr::Var("f"),
                Expr::Abs(
                    "v",
                    Expr::App(Expr::App(Expr::Var("x"), Expr::Var("x")), Expr::Var("v")),
                ),
            ),
        );
        let y_comb = Expr::Abs("f", Expr::App(inner.clone(), inner));

        let fib_norec = Expr::Abs(
            "x",
            Expr::Conditional(
                Expr::Comparison(ComparisonOp::Lt, Expr::Var("x"), Expr::NumericLiteral(2.)),
                Expr::NumericLiteral(1.),
                Expr::Arithmetic(
                    ArithmeticOp::Add,
                    Expr::App(
                        Expr::Var("f"),
                        Expr::Arithmetic(
                            ArithmeticOp::Sub,
                            Expr::Var("x"),
                            Expr::NumericLiteral(1.),
                        ),
                    ),
                    Expr::App(
                        Expr::Var("f"),
                        Expr::Arithmetic(
                            ArithmeticOp::Sub,
                            Expr::Var("x"),
                            Expr::NumericLiteral(2.),
                        ),
                    ),
                ),
            ),
        );

        let fib = Expr::App(y_comb, fib_norec);

        let fib_5 = Expr::App(fib, Expr::NumericLiteral(5.));

        assert_eq!(fib_5.eval(), Ok(EvalResult::Value(8.)));
    }
}
