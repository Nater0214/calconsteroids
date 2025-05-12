use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::{clone_trait_object, DynClone};

pub use addition::AdditionExpression;
pub use division::DivisionExpression;
pub use multiplication::MultiplicationExpression;
pub use subtraction::SubtractionExpression;
pub use value::ValueExpression;

mod addition;
mod division;
mod multiplication;
mod subtraction;
mod value;

/// A mathematical expression
pub trait Expression: Downcast + DynClone + Debug {
    /// Returns the simplified version of this expression
    fn simplified(&self) -> Box<dyn Expression>;

    /// Get a string representation of this expression
    fn to_string(&self) -> String;
}
impl_downcast!(Expression);
clone_trait_object!(Expression);
