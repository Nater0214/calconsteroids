use super::{Expression, ValueExpression};

/// An addition expression
#[derive(Debug, Clone)]
pub struct AdditionExpression {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl AdditionExpression {
    pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expression for AdditionExpression {
    fn simplified(&self) -> Box<dyn Expression> {
        // Simplify both sides
        let lhs = self.lhs.simplified();
        let rhs = self.rhs.simplified();

        // Combine if two values
        if let (Some(lhs), Some(rhs)) = (
            lhs.downcast_ref::<ValueExpression>(),
            rhs.downcast_ref::<ValueExpression>(),
        ) {
            Box::new(ValueExpression::new(lhs.get_value().add(rhs.get_value())))
        } else {
            Box::new(AdditionExpression::new(lhs, rhs))
        }
    }

    fn to_string(&self) -> String {
        format!("({} + {})", self.lhs.to_string(), self.rhs.to_string())
    }
}
