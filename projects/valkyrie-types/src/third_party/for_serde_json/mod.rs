use super::*;
use nyar_error::third_party::JsonValue;
use std::convert::Infallible;

impl ValkyrieType for JsonValue {
    fn boxed(self) -> ValkyrieValue {
        match self {
            JsonValue::Null => ValkyrieValue::Null,
            JsonValue::Bool(v) => ValkyrieValue::Boolean(v),
            JsonValue::Number(v) => {
                todo!()
            }
            JsonValue::String(v) => ValkyrieValue::UTF8String(Gc::new(v)),
            JsonValue::Array(v) => {
                todo!()
            }
            JsonValue::Object(v) => {
                todo!()
            }
        }
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        todo!()
    }
}
