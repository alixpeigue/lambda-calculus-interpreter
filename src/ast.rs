use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Abs {
        var: Rc<str>,
        body: Rc<Expr>,
    },
    App {
        function: Rc<Expr>,
        parameter: Rc<Expr>,
    },
    Arithmetic {
        operation: ArithmeticOp,
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Comparison {
        operation: ComparisonOp,
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Conditional {
        condition: Rc<Expr>,
        true_branch: Rc<Expr>,
        false_branch: Rc<Expr>,
    },
    Var {
        name: Rc<str>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
}

impl Expr {
    pub fn Abs(var: &str, body: Expr) -> Self {
        Expr::Abs {
            var: Rc::from(var),
            body: Rc::new(body),
        }
    }

    pub fn App(function: Expr, parameter: Expr) -> Self {
        Expr::App {
            function: Rc::new(function),
            parameter: Rc::new(parameter),
        }
    }

    pub fn Arithmetic(operation: ArithmeticOp, lhs: Expr, rhs: Expr) -> Self {
        Expr::Arithmetic {
            operation,
            lhs: Rc::new(lhs),
            rhs: Rc::new(rhs),
        }
    }

    pub fn Comparison(operation: ComparisonOp, lhs: Expr, rhs: Expr) -> Self {
        Expr::Comparison {
            operation,
            lhs: Rc::new(lhs),
            rhs: Rc::new(rhs),
        }
    }

    pub fn Conditional(condition: Expr, true_branch: Expr, false_branch: Expr) -> Self {
        Expr::Conditional {
            condition: Rc::new(condition),
            true_branch: Rc::new(true_branch),
            false_branch: Rc::new(false_branch),
        }
    }

    pub fn Var(name: &str) -> Self {
        Expr::Var {
            name: Rc::from(name),
        }
    }

    pub fn NumericLiteral(value: f64) -> Self {
        Expr::NumericLiteral { value }
    }

    pub fn BooleanLiteral(value: bool) -> Self {
        Expr::BooleanLiteral { value }
    }
}
