use super::*;

impl<'a> IntoIterator for &'a FlagDeclaration {
    type Item = FlagTerm;
    type IntoIter = FlagIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FlagIterator { inner: self.body.terms.iter() }
    }
}

impl<'a> Iterator for FlagIterator<'a> {
    type Item = FlagTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.inner.next()?;
        match &term {
            StatementNode::EnumerateField(field) => Some(FlagTerm::Field((**field).clone())),
            _ => self.next(),
        }
    }
}
