use crate::value::Value;

use super::Expression;

/// An expression that contains a value
#[derive(Debug, Clone)]
pub struct ValueExpression {
    value: Box<dyn Value>,
}

impl ValueExpression {
    pub fn new(value: Box<dyn Value>) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> &dyn Value {
        self.value.as_ref()
    }
}

impl Expression for ValueExpression {
    /// Simplify this expression
    fn simplified(&self) -> Box<dyn Expression> {
        Box::new(ValueExpression::new(self.value.clone()))
    }

    /// Get a string representation of this expression
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
