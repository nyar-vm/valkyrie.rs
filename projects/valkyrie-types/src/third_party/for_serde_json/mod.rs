use crate::{types::ValkyrieMetaType, ValkyrieError, ValkyrieType, ValkyrieValue};
use serde_json::{Error, Value};
use std::sync::Arc;

impl ValkyrieType for Value {
    fn boxed(self) -> ValkyrieValue {
        match self {
            Value::Null => ValkyrieValue::Null,
            Value::Bool(v) => ValkyrieValue::Boolean(v),
            Value::Number(v) => {
                if v.is_i64() {
                    ValkyrieValue::Integer(v.as_i64().unwrap().into())
                }
                else if v.is_u64() {
                    ValkyrieValue::Integer(v.as_u64().unwrap().into())
                }
                else if v.is_f64() {
                    ValkyrieValue::Float64(v.as_f64().unwrap())
                }
                else {
                    todo!()
                }
            }
            Value::String(v) => ValkyrieValue::UTF8String(Arc::new(v)),
            Value::Array(_) => {
                todo!()
            }
            Value::Object(_) => {
                todo!()
            }
        }
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}

impl From<Error> for ValkyrieError {
    fn from(value: Error) -> Self {
        ValkyrieError::custom(value.to_string())
    }
}
