use std::{collections::HashMap, fmt};

use crate::value::Value;

/// A map of strings to variables
pub type VariableMap = HashMap<String, Value>;

/// An expression
#[derive(Debug, Clone)]
pub enum Expression {
    /// A value
    Value(Value),
    /// A variable
    Variable(String),
    /// An addition expression
    Addition(Box<Expression>, Box<Expression>),
    /// A subtraction expression
    Subtraction(Box<Expression>, Box<Expression>),
    /// A multiplication expression
    Multiplication(Box<Expression>, Box<Expression>),
    /// A division expression
    Division(Box<Expression>, Box<Expression>),
    /// A negation
    Negation(Box<Expression>),
    /// A factorial
    Factorial(Box<Expression>),
}

impl Expression {
    /// Return the one step simplified version of this expression
    pub fn simplified(&self) -> Self {
        match self {
            Expression::Value(v) => Expression::Value(v.to_owned()),
            Expression::Variable(v) => Expression::Variable(v.to_owned()),
            Expression::Addition(a, b) => {
                let a = a.simplified();
                let b = b.simplified();
                if let (Expression::Value(a), Expression::Value(b)) = (&a, &b) {
                    Expression::Value(a.to_owned() + b.to_owned())
                } else {
                    Expression::Addition(Box::new(a), Box::new(b))
                }
            }
            Expression::Subtraction(a, b) => {
                let a = a.simplified();
                let b = b.simplified();
                if let (Expression::Value(a), Expression::Value(b)) = (&a, &b) {
                    Expression::Value(a.to_owned() - b.to_owned())
                } else {
                    Expression::Subtraction(Box::new(a), Box::new(b))
                }
            }
            Expression::Multiplication(a, b) => {
                let a = a.simplified();
                let b = b.simplified();
                if let (Expression::Value(a), Expression::Value(b)) = (&a, &b) {
                    Expression::Value(a.to_owned() * b.to_owned())
                } else {
                    Expression::Multiplication(Box::new(a), Box::new(b))
                }
            }
            Expression::Division(a, b) => {
                let a = a.simplified();
                let b = b.simplified();
                if let (Expression::Value(a), Expression::Value(b)) = (&a, &b) {
                    Expression::Value(a.to_owned() / b.to_owned())
                } else {
                    Expression::Division(Box::new(a), Box::new(b))
                }
            }
            Expression::Negation(expression) => {
                Expression::Negation(Box::new(expression.simplified()))
            }
            Expression::Factorial(expression) => {
                Expression::Factorial(Box::new(expression.simplified()))
            }
        }
    }

    /// Simplify the expression one step in place
    #[inline]
    pub fn simplify(&mut self) {
        *self = self.simplified();
    }

    /// Evaluate the expression
    pub fn evaluate(&self, variable_map: &VariableMap) -> Value {
        match self {
            Expression::Value(v) => v.to_owned(),
            Expression::Variable(n) => variable_map.get(n).unwrap_or(&Value::Undefined).to_owned(),
            Expression::Addition(a, b) => a.evaluate(variable_map) + b.evaluate(variable_map),
            Expression::Subtraction(a, b) => a.evaluate(variable_map) - b.evaluate(variable_map),
            Expression::Multiplication(a, b) => a.evaluate(variable_map) * b.evaluate(variable_map),
            Expression::Division(a, b) => a.evaluate(variable_map) / b.evaluate(variable_map),
            Expression::Negation(expression) => -expression.evaluate(variable_map),
            Expression::Factorial(expression) => expression.evaluate(variable_map).factorial(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Value(v) => write!(f, "{}", v),
            Expression::Variable(n) => write!(f, "{}", n),
            Expression::Addition(a, b) => write!(f, "({a} + {b})"),
            Expression::Subtraction(a, b) => write!(f, "({a} - {b})"),
            Expression::Multiplication(a, b) => match (*a.to_owned(), *b.to_owned()) {
                (Expression::Value(a), Expression::Variable(b)) => write!(f, "{a}{b}"),
                (Expression::Variable(a), Expression::Variable(b)) => write!(f, "{a}{b}"),
                (Expression::Value(a), b) => write!(f, "{a}{b}"),
                (a, b) => write!(f, "({a} * {b})"),
            }
            Expression::Division(a, b) => write!(f, "({a} / {b})"),
            Expression::Negation(expression) => write!(f, "-({expression})"),
            Expression::Factorial(expression) => write!(f, "({expression})!"),
        }
    }
}
