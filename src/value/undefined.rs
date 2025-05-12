use super::Value;

#[derive(Debug, Clone)]
pub struct UndefinedValue;

impl UndefinedValue {
    pub fn new() -> Self {
        Self
    }
}

impl Value for UndefinedValue {
    fn add(&self, _other: &dyn Value) -> Box<(dyn Value + 'static)> {
        Box::new(UndefinedValue::new())
    }

    fn sub(&self, _other: &dyn Value) -> Box<(dyn Value + 'static)> {
        Box::new(UndefinedValue::new())
    }

    fn mul(&self, _other: &dyn Value) -> Box<(dyn Value + 'static)> {
        Box::new(UndefinedValue::new())
    }

    fn div(&self, _other: &dyn Value) -> Box<(dyn Value + 'static)> {
        Box::new(UndefinedValue::new())
    }

    fn cmp(&self, _other: &dyn Value) -> Option<std::cmp::Ordering> {
        None
    }

    fn to_string(&self) -> String {
        "undefined".to_string()
    }
}
