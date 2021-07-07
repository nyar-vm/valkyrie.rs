use super::*;

impl Default for ImportTermNode {
    fn default() -> Self {
        Self::Group(Default::default())
    }
}

impl FromIterator<IdentifierNode> for ImportGroupNode {
    fn from_iter<T: IntoIterator<Item = IdentifierNode>>(iter: T) -> Self {
        Self { path: iter.into_iter().collect(), terms: vec![] }
    }
}
impl FromIterator<ImportTermNode> for ImportGroupNode {
    fn from_iter<T: IntoIterator<Item = ImportTermNode>>(iter: T) -> Self {
        Self { path: vec![], terms: iter.into_iter().collect() }
    }
}
