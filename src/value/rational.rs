use std::{ops::BitXor, str::FromStr, string::ParseError};

use num::{bigint::ParseBigIntError, BigUint, Integer as _};

use super::{UndefinedValue, Value};

/// The sign of a rational value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    /// Return the opposite of this sign
    pub fn opposite(&self) -> Self {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

impl BitXor for Sign {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative,
            _ => Sign::Positive,
        }
    }
}

impl AsRef<bool> for Sign {
    fn as_ref(&self) -> &bool {
        match self {
            Sign::Positive => &false,
            Sign::Negative => &true,
        }
    }
}

impl Into<bool> for Sign {
    fn into(self) -> bool {
        match self {
            Sign::Positive => false,
            Sign::Negative => true,
        }
    }
}

impl From<bool> for Sign {
    fn from(value: bool) -> Self {
        match value {
            false => Sign::Positive,
            true => Sign::Negative,
        }
    }
}

/// A rational value
#[derive(Debug, Clone)]
pub struct RationalValue {
    sign: Sign,
    numerator: BigUint,
    denominator: BigUint,
}

impl RationalValue {
    /// Construct a new rational value
    ///
    /// If sign is true, the number is negative
    pub fn new(sign: Sign, numerator: impl Into<BigUint>, denominator: impl Into<BigUint>) -> Self {
        Self {
            sign,
            numerator: numerator.into(),
            denominator: denominator.into(),
        }
    }

    /// Get the sign of this rational value
    pub fn get_sign(&self) -> &Sign {
        &self.sign
    }

    /// Get the numerator of this rational value
    pub fn get_numerator(&self) -> &BigUint {
        &self.numerator
    }

    /// Get the denominator of this rational value
    pub fn get_denominator(&self) -> &BigUint {
        &self.denominator
    }

    /// Return the simplified version of this rational value
    pub fn simplified(&self) -> Self {
        let gcd = self.numerator.gcd(&self.denominator);
        Self::new(
            self.sign,
            self.get_numerator() / &gcd,
            self.get_denominator() / &gcd,
        )
    }

    pub fn get_opposite(&self) -> Self {
        Self::new(
            self.sign.opposite(),
            self.numerator.clone(),
            self.denominator.clone(),
        )
    }

    pub fn get_reciprocal(&self) -> Self {
        Self::new(self.sign, self.denominator.clone(), self.numerator.clone())
    }
}

impl Value for RationalValue {
    fn add(&self, other: &dyn Value) -> Box<dyn Value> {
        if let Some(other) = other.downcast_ref::<RationalValue>() {
            if *other.get_sign() == Sign::Negative {
                self.sub(&other.get_opposite())
            } else if *self.get_sign() == Sign::Negative {
                other.sub(&self.get_opposite())
            } else {
                Box::new(
                    RationalValue::new(
                        Sign::Positive,
                        self.get_numerator() * other.get_denominator()
                            + other.get_numerator() * self.get_denominator(),
                        self.get_denominator() * other.get_denominator(),
                    )
                    .simplified(),
                )
            }
        } else {
            Box::new(UndefinedValue::new())
        }
    }

    fn sub(&self, other: &dyn Value) -> Box<dyn Value> {
        if let Some(other) = other.downcast_ref::<RationalValue>() {
            if *other.get_sign() == Sign::Negative {
                self.add(&other.get_opposite())
            } else if *self.get_sign() == Sign::Negative {
                if let Some(sum) = self
                    .add(&other.get_opposite())
                    .downcast_ref::<RationalValue>()
                {
                    Box::new(sum.get_opposite())
                } else {
                    panic!("Unexpected error: adding two rational values didn't yield a rational value!")
                }
            } else {
                Box::new(
                    RationalValue::new(
                        Sign::Positive,
                        self.get_numerator() * other.get_denominator()
                            - other.get_numerator() * self.get_denominator(),
                        self.get_denominator() * other.get_denominator(),
                    )
                    .simplified(),
                )
            }
        } else {
            Box::new(UndefinedValue::new())
        }
    }

    fn mul(&self, other: &dyn Value) -> Box<dyn Value> {
        if let Some(other) = other.downcast_ref::<RationalValue>() {
            Box::new(RationalValue::new(
                *self.get_sign() ^ *other.get_sign(),
                self.get_numerator() * other.get_numerator(),
                self.get_denominator() * other.get_denominator(),
            ))
        } else {
            Box::new(UndefinedValue::new())
        }
    }

    fn div(&self, other: &dyn Value) -> Box<dyn Value> {
        if let Some(other) = other.downcast_ref::<RationalValue>() {
            self.mul(&other.get_reciprocal())
        } else {
            Box::new(UndefinedValue::new())
        }
    }

    fn cmp(&self, other: &dyn Value) -> Option<std::cmp::Ordering> {
        if let Some(other) = other.downcast_ref::<RationalValue>() {
            Some(
                (self.get_numerator() * other.get_denominator())
                    .cmp(&(other.get_numerator() * self.get_denominator())),
            )
        } else {
            None
        }
    }

    fn to_string(&self) -> String {
        if self.denominator == BigUint::from(1u32) {
            format!(
                "{}{}",
                if self.sign.into() { "-" } else { "" },
                self.numerator
            )
        } else {
            format!(
                "{}{}/{}",
                if self.sign.into() { "-" } else { "" },
                self.numerator,
                self.denominator
            )
        }
    }
}

impl FromStr for RationalValue {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('-') {
            if let Some((before, after)) = s.split_once(".") {
                let combined = format!("{}{}", before, after);
                let combined = combined.trim_end_matches('0');
                let numerator = BigUint::from_str(combined)?;
                let denominator = BigUint::from(after.len());
                Ok(Self::new(Sign::Negative, numerator, denominator))
            } else {
                let numerator = BigUint::from_str(s)?;
                Ok(Self::new(Sign::Negative, numerator, BigUint::from(1u32)))
            }
        } else {
            if let Some((before, after)) = s.split_once(".") {
                let combined = format!("{}{}", before, after);
                let combined = combined.trim_end_matches('0');
                let numerator = BigUint::from_str(combined)?;
                let denominator = BigUint::from(after.len());
                Ok(Self::new(Sign::Positive, numerator, denominator))
            } else {
                let numerator = BigUint::from_str(s)?;
                Ok(Self::new(Sign::Positive, numerator, BigUint::from(1u32)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::{RationalValue, Sign};

    #[test]
    fn simplify() {
        let value = RationalValue::new(Sign::Positive, 6_u32, 4_u32);
        assert_eq!(
            value
                .simplified()
                .cmp(&RationalValue::new(Sign::Positive, 3_u32, 2_u32)),
            Some(std::cmp::Ordering::Equal)
        );
    }
}
