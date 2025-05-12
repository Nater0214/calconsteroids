use super::{Expression, ValueExpression};

/// A subtraction expression
#[derive(Debug, Clone)]
pub struct SubtractionExpression {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl SubtractionExpression {
    pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expression for SubtractionExpression {
    fn simplified(&self) -> Box<dyn Expression> {
        // Simplify both sides
        let lhs = self.lhs.simplified();
        let rhs = self.rhs.simplified();

        // Combine if two values
        if let (Some(lhs), Some(rhs)) = (
            lhs.downcast_ref::<ValueExpression>(),
            rhs.downcast_ref::<ValueExpression>(),
        ) {
            Box::new(ValueExpression::new(lhs.get_value().sub(rhs.get_value())))
        } else {
            Box::new(SubtractionExpression::new(lhs, rhs))
        }
    }

    fn to_string(&self) -> String {
        format!("({} - {})", self.lhs.to_string(), self.rhs.to_string())
    }
}
