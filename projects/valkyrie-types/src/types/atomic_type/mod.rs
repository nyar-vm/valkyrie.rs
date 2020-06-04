use super::*;

// type without generic
#[derive(Debug)]
pub struct ValkyrieAtomicType {
    namepath: Vec<String>,
    document: ValkyrieDocument,
}
#[derive(Debug, Default)]
pub struct ValkyrieDocument {
    input: String,
}

impl ValkyrieAtomicType {
    pub fn new(namepath: &str) -> Self {
        Self { namepath: namepath.split('.').map(|s| s.to_string()).collect(), document: ValkyrieDocument::default() }
    }
}

impl Into<ValkyrieAtomicType> for u8 {
    fn into(self) -> ValkyrieAtomicType {
        ValkyrieAtomicType::new("std.primitive.Unsigned8")
    }
}
