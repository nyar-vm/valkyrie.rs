use super::*;
use crate::values::ValkyrieType;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ValkyrieText {
    character: bool,
    encoding: &'static str,
    buffer: Vec<u8>,
}

impl ValkyrieValueType for ValkyrieText {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Text(self.clone())
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Text { character: self.character, encoding: self.encoding }
    }
}

impl ValkyrieValueType for char {
    fn as_valkyrie(&self) -> ValkyrieValue {
        let c = *self as u32;
        ValkyrieValue::Text(ValkyrieText { character: true, encoding: "Unicode", buffer: c.to_le_bytes().to_vec() })
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Text { character: true, encoding: "Unicode" }
    }
}

impl<'a> ValkyrieValueType for &'a str {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Text(ValkyrieText { character: false, encoding: "Utf8Text", buffer: self.as_bytes().to_vec() })
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Text { character: false, encoding: "Utf8Text" }
    }
}

impl ValkyrieValueType for String {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Text(ValkyrieText { character: false, encoding: "Utf8Text", buffer: self.as_bytes().to_vec() })
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Text { character: false, encoding: "Utf8Text" }
    }
}
