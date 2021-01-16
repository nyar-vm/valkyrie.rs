use super::*;

impl ValkyrieType for JsonValue {
    fn boxed(self) -> ValkyrieValue {
        match self {
            JsonValue::Null => ValkyrieValue::Null,
            JsonValue::Bool(v) => ValkyrieValue::Boolean(v),
            JsonValue::Number(v) => {
                if let Some(s) = v.as_i64() {
                    return ValkyrieValue::from(s);
                }
                if let Some(s) = v.as_u64() {
                    return ValkyrieValue::from(s);
                }
                if let Some(s) = v.as_f64() {
                    return ValkyrieValue::from(s);
                }
                ValkyrieValue::Null
            }
            JsonValue::String(v) => ValkyrieValue::UTF8String(Gc::new(v)),
            JsonValue::Array(_) => {
                todo!()
            }
            JsonValue::Object(_) => {
                todo!()
            }
        }
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        todo!()
    }
}
