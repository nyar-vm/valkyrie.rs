use super::*;
use crate::NumberNode;

impl PartialEq for IdentifierNode {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl PartialEq for ValkyrieOperator {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl PartialEq for NamepathNode {
    fn eq(&self, other: &Self) -> bool {
        self.names.eq(&other.names)
    }
}
impl PartialEq for NumberNode {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.unit.eq(&other.unit)
    }
}
