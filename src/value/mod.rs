use std::{cmp::Ordering, fmt::Debug};

use downcast_rs::{impl_downcast, Downcast};

use dyn_clone::{clone_trait_object, DynClone};

pub use rational::RationalValue;
pub use undefined::UndefinedValue;

mod rational;
mod undefined;

/// The root trait for all values
pub trait Value: Downcast + DynClone + Debug {
    /// Add this value with another value
    fn add(&self, other: &dyn Value) -> Box<dyn Value>;

    /// Subtract another value from this value
    fn sub(&self, other: &dyn Value) -> Box<dyn Value>;

    /// Multiply this value by another value
    fn mul(&self, other: &dyn Value) -> Box<dyn Value>;

    /// Divide this value by another value
    fn div(&self, other: &dyn Value) -> Box<dyn Value>;

    /// Compare this value to another value
    fn cmp(&self, other: &dyn Value) -> Option<Ordering>;

    /// Get a string representation of this value
    fn to_string(&self) -> String;
}
impl_downcast!(Value);
clone_trait_object!(Value);
