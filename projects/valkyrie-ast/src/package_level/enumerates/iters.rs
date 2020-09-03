use super::*;

impl<'a> IntoIterator for &'a EnumerateDeclaration {
    type Item = EnumerateTerm;
    type IntoIter = EnumerateIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EnumerateIterator { iter: self.statements.terms.iter() }
    }
}

impl<'a> Iterator for EnumerateIterator<'a> {
    type Item = EnumerateTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.iter.next()?;
        match &term.r#type {
            StatementBody::Variant(variant) => Some(EnumerateTerm::Variant((**variant).clone())),
            _ => None,
        }
    }
}

impl StatementNode {
    /// Check if the statement is a variant node.
    #[inline]
    pub fn is_variant(&self) -> bool {
        matches!(self.r#type, StatementBody::Variant(_))
    }
}
