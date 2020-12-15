use super::*;

impl<'a> IntoIterator for &'a EnumerateDeclaration {
    type Item = EnumerateTerm;
    type IntoIter = EnumerateIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EnumerateIterator { inner: self.body.terms.iter() }
    }
}

impl<'a> Iterator for EnumerateIterator<'a> {
    type Item = EnumerateTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.inner.next()?;
        match &term.r#type {
            StatementType::EnumerateField(field) => Some(EnumerateTerm::Field((**field).clone())),
            _ => self.next(),
        }
    }
}
