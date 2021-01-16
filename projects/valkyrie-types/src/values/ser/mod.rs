use crate::ValkyrieValue;
use serde::{Serialize, Serializer};

impl Serialize for ValkyrieValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Uninitialized => {
                todo!()
            }
            Self::Nothing => {
                todo!()
            }
            Self::Null => serializer.serialize_none(),
            Self::Unit => serializer.serialize_unit(),
            Self::Boolean(v) => serializer.serialize_bool(*v),
            Self::Number(v) => v.serialize(serializer),
            Self::Unicode(v) => serializer.serialize_char(*v),
            Self::UTF8String(v) => serializer.serialize_str(v),
            Self::Bytes(_) => {
                todo!()
            }
            Self::Class(_) => {
                todo!()
            }
            Self::Variant(_) => {
                todo!()
            }
            Self::NDArray(_) => {
                todo!()
            }
            Self::Image(_) => {
                todo!()
            }
            #[cfg(feature = "polars")]
            Self::DataFrame(_) => {
                todo!()
            }
            Self::Table(_) => {
                todo!()
            }
            Self::Html(_) => {
                todo!()
            }
        }
    }
}
