use super::*;

impl FromIterator<IdentifierNode> for ImportGroupNode {
    fn from_iter<T: IntoIterator<Item = IdentifierNode>>(iter: T) -> Self {
        Self { path: iter.into_iter().collect(), group: vec![] }
    }
}
impl FromIterator<ImportTermNode> for ImportGroupNode {
    fn from_iter<T: IntoIterator<Item = ImportTermNode>>(iter: T) -> Self {
        Self { path: vec![], group: iter.into_iter().collect() }
    }
}
