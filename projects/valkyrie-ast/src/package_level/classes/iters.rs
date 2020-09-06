use super::*;

impl<'a> IntoIterator for &'a ClassDeclaration {
    type Item = ClassTerm;
    type IntoIter = ClassIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ClassIterator { inner: self.statements.iter() }
    }
}

impl<'a> Iterator for ClassIterator<'a> {
    type Item = ClassTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.inner.next()?;
        match &term.r#type {
            StatementBody::ClassField(field) => Some(ClassTerm::Field((**field).clone())),
            StatementBody::ClassMethod(method) => Some(ClassTerm::Method((**method).clone())),
            _ => None,
        }
    }
}
