use super::*;

impl PartialEq for ValkyrieIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl PartialEq for ValkyrieOperator {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl PartialEq for ValkyrieNamepath {
    fn eq(&self, other: &Self) -> bool {
        self.names.eq(&other.names)
    }
}
