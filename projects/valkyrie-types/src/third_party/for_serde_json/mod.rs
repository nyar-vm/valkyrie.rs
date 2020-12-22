use super::*;
use dashu::float::FBig;

impl ValkyrieType for JsonValue {
    fn boxed(self) -> ValkyrieValue {
        match self {
            JsonValue::Null => ValkyrieValue::Null,
            JsonValue::Bool(v) => ValkyrieValue::Boolean(v),
            JsonValue::Number(v) => {
                if v.is_i64() {
                    ValkyrieValue::Integer(v.as_i64().unwrap().into())
                }
                else if v.is_u64() {
                    ValkyrieValue::Integer(v.as_u64().unwrap().into())
                }
                else if v.is_f64() {
                    ValkyrieValue::Decimal(FBig::try_from(v.as_f64().unwrap()).unwrap())
                }
                else {
                    todo!()
                }
            }
            JsonValue::String(v) => ValkyrieValue::UTF8String(Arc::new(v)),
            JsonValue::Array(_) => {
                todo!()
            }
            JsonValue::Object(_) => {
                todo!()
            }
        }
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}
