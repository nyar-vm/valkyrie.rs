use super::*;

impl PartialEq for NamespaceDeclareNode {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind) && self.path.eq(&other.path)
    }
}
