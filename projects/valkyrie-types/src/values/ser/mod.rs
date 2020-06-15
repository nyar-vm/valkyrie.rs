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
            ValkyrieValue::Unsigned8(v) => serializer.serialize_u8(*v),
            ValkyrieValue::Unsigned16(v) => serializer.serialize_u16(*v),
            ValkyrieValue::Unsigned32(v) => serializer.serialize_u32(*v),
            ValkyrieValue::Unsigned64(v) => serializer.serialize_u64(*v),
            ValkyrieValue::Unsigned128(v) => serializer.serialize_u128(*v),
            ValkyrieValue::Integer8(v) => serializer.serialize_i8(*v),
            ValkyrieValue::Integer16(v) => serializer.serialize_i16(*v),
            ValkyrieValue::Integer32(v) => serializer.serialize_i32(*v),
            ValkyrieValue::Integer64(v) => serializer.serialize_i64(*v),
            ValkyrieValue::Integer128(v) => serializer.serialize_i128(*v),
            ValkyrieValue::Float32(v) => serializer.serialize_f32(*v),
            ValkyrieValue::Float64(v) => serializer.serialize_f64(*v),
            ValkyrieValue::String(v) => serializer.serialize_str(v),
            ValkyrieValue::Buffer(_) => {
                todo!()
            }
            ValkyrieValue::Class(_) => {
                todo!()
            }
            ValkyrieValue::Variant(_) => {
                todo!()
            }
        }
    }
}
