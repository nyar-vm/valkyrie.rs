use super::*;

impl<'a> IntoIterator for &'a FlagsDeclaration {
    type Item = FlagsTerm;
    type IntoIter = FlagsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FlagsIterator { inner: self.body.terms.iter() }
    }
}

impl<'a> Iterator for FlagsIterator<'a> {
    type Item = FlagsTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.inner.next()?;
        match &term.r#type {
            StatementBody::EnumerateField(field) => Some(FlagsTerm::Field((**field).clone())),
            _ => self.next(),
        }
    }
}
