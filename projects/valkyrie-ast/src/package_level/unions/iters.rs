use super::*;

impl<'a> IntoIterator for &'a UnionDeclaration {
    type Item = &'a UnionTerm;
    type IntoIter = core::slice::Iter<'a, UnionTerm>;

    fn into_iter(self) -> Self::IntoIter {
        self.body.iter()
    }
}
