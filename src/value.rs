use num::integer::{gcd, lcm};
use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use crate::expression::{Expression, VariableMap};

/// An error on a value
#[derive(Debug)]
pub enum ValueError {
    /// A bad variant of `Value` was given
    BadVariant(Value),
    /// A bad value was given
    BadValue(Value),
}

/// A result of a value operation
pub type ValueResult = Result<Value, ValueError>;

/// A value
#[derive(Debug, Clone)]
pub enum Value {
    Rational(i128, i128),
    Expression(Box<Expression>),
    Undefined,
}

/// Simplifies the given `Value::Rational` in place
///
/// Does nothing if the value is not a `Value::Rational`
#[allow(dead_code)]
fn simplify_rational_mut(value: &mut Value) {
    if let Value::Rational(numerator, denominator) = value {
        let gcf = gcd(*numerator, *denominator);
        *numerator /= gcf;
        *denominator /= gcf;
    }
}

/// Simplify the given `Value::Rational` and return
fn simplify_rational(value: Value) -> Result<Value, ValueError> {
    if let Value::Rational(numerator, denominator) = value {
        let gcf = gcd(numerator, denominator);
        Ok(Value::Rational(numerator / gcf, denominator / gcf))
    } else {
        Err(ValueError::BadVariant(value))
    }
}

impl From<i128> for Value {
    #[inline]
    fn from(value: i128) -> Self {
        Self::Rational(value, 1)
    }
}

/// An error in parsing a `Value` from a string
#[derive(Debug)]
#[allow(dead_code)]
pub struct ValueParseError {
    /// The string that failed to parse
    string: String,
}

impl FromStr for Value {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string at a decimal point if any and get the two parts
        let (integer, decimal) = match s.split_once('.') {
            Some((integer, decimal)) => (integer, Some(decimal)),
            None => (s, None),
        };

        // Parse the integer part
        let integer = integer.parse::<i128>().map_err(|_| ValueParseError {
            string: s.to_string(),
        })?;

        // Parse the decimal part if any
        let decimal = match decimal {
            Some(decimal) => decimal.parse::<i128>().map_err(|_| ValueParseError {
                string: s.to_string(),
            })?,
            None => 0,
        };

        // Determine the exponent to multiply the value by to make it an integer
        // This is done by taking the common log of the decimal part
        let exponent = decimal.checked_ilog10().unwrap_or(0);

        // Determine the numerator
        // This is done by multiplying the integer part by 10^exponent and adding the decimal part
        let numerator = integer * 10_i128.pow(exponent) + decimal;

        // Determine the denominator
        // This is done by evaluating 10^exponent
        let denominator = 10_i128.pow(exponent);

        // Return the rational value
        Ok(Value::Rational(numerator, denominator))
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::Rational(numerator, denominator) => numerator as f64 / denominator as f64,
            Value::Undefined => f64::NAN,
            Value::Expression(expression) => expression.evaluate(&VariableMap::new()).into(),
        }
    }
}

impl Add for Value {
    type Output = Self;

    /// Add two values together by performing the `+` operation
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Rational(self_num, self_den), Value::Rational(other_num, other_den)) => {
                let common_den = lcm(self_den, other_den);
                let self_num = self_num * (common_den / self_den);
                let other_num = other_num * (common_den / other_den);
                simplify_rational(Self::Rational(self_num + other_num, common_den)).unwrap()
            }
            _ => Self::Undefined,
        }
    }
}

impl Sub for Value {
    type Output = Self;

    /// Subtract two values by performing the `-` operation
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Rational(self_num, self_den), Value::Rational(other_num, other_den)) => {
                let common_den = lcm(self_den, other_den);
                let self_num = self_num * (common_den / self_den);
                let other_num = other_num * (common_den / other_den);
                simplify_rational(Self::Rational(self_num - other_num, common_den)).unwrap()
            }
            _ => Self::Undefined,
        }
    }
}

impl Mul for Value {
    type Output = Self;

    /// Multiply two values by performing the `*` operation
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Rational(self_num, self_den), Value::Rational(other_num, other_den)) => {
                simplify_rational(Self::Rational(self_num * other_num, self_den * other_den))
                    .unwrap()
            }
            _ => Self::Undefined,
        }
    }
}

impl Div for Value {
    type Output = Self;

    /// Divide two values by performing the `/` operation
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Rational(self_num, self_den), Value::Rational(other_num, other_den)) => {
                simplify_rational(Self::Rational(self_num * other_den, self_den * other_num))
                    .unwrap()
            }
            _ => Self::Undefined,
        }
    }
}

impl Neg for Value {
    type Output = Self;

    /// Negate a value by performing the `-` operation
    fn neg(self) -> Self {
        match self {
            Value::Rational(numerator, denominator) => {
                simplify_rational(Self::Rational(-numerator, denominator)).unwrap()
            }
            _ => Self::Undefined,
        }
    }
}

impl Value {
    /// Factorial a value
    pub fn factorial(self) -> Value {
        match self {
            Value::Rational(numerator, denominator) => {
                if numerator < 0 || denominator != 1 {
                    return Value::Undefined;
                }

                let mut result = 1;
                for i in 2..=numerator {
                    result *= i;
                }

                Value::Rational(result, denominator)
            }
            _ => Value::Undefined,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Value::Rational(self_num, self_den), &Value::Rational(other_num, other_den)) => {
                self_num * other_den == other_num * self_den
            }
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Rational(numerator, denominator) => {
                if *denominator == 1 {
                    write!(f, "{numerator}")
                } else {
                    write!(f, "{numerator}/{denominator}")
                }
            }
            Value::Expression(expression) => write!(f, "{expression}"),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}
