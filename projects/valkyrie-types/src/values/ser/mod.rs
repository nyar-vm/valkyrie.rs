use crate::ValkyrieValue;
use serde::{Serialize, Serializer};

impl Serialize for ValkyrieValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValkyrieValue::Nothing => {
                todo!()
            }
            ValkyrieValue::Null => serializer.serialize_none(),
            ValkyrieValue::Unit => serializer.serialize_unit(),
            ValkyrieValue::Boolean(v) => serializer.serialize_bool(*v),
            ValkyrieValue::Integer(v) => v.serialize(serializer),
            ValkyrieValue::Decimal(v) => serializer.serialize_f64(*v),
            ValkyrieValue::UTF8Character(v) => serializer.serialize_char(*v),
            ValkyrieValue::UTF8String(v) => serializer.serialize_str(v),
            ValkyrieValue::Bytes(_) => {
                todo!()
            }
            ValkyrieValue::Class(_) => {
                todo!()
            }
            ValkyrieValue::Variant(_) => {
                todo!()
            }
            ValkyrieValue::Json(_) => {
                todo!()
            }
            ValkyrieValue::NDArray(_) => {
                todo!()
            }
            ValkyrieValue::Image(_) => {
                todo!()
            }
            ValkyrieValue::DataFrame(_) => {
                todo!()
            }
            ValkyrieValue::Table(_) => {
                todo!()
            }
            ValkyrieValue::Html(_) => {
                todo!()
            }
        }
    }
}
