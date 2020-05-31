use super::*;

impl PartialEq for IdentifierNode {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl PartialEq for OperatorNode {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl PartialEq for NamepathNode {
    fn eq(&self, other: &Self) -> bool {
        self.names.eq(&other.names)
    }
}
impl PartialEq for NumberLiteralNode {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.unit.eq(&other.unit)
    }
}
impl PartialEq for StringLiteralNode {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.unit.eq(&other.unit)
    }
}
