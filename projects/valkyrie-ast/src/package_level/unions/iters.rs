use super::*;

impl<'a> IntoIterator for &'a UnionDeclaration {
    type Item = UnionTerm;
    type IntoIter = UnionIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        UnionIterator { inner: self.body.terms.iter() }
    }
}

impl<'a> Iterator for UnionIterator<'a> {
    type Item = UnionTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.inner.next()?;
        match &term.r#type {
            StatementType::UnionField(field) => Some(UnionTerm::Field((**field).clone())),
            _ => self.next(),
        }
    }
}
