use super::*;

impl<'a> IntoIterator for &'a TaggedDeclaration {
    type Item = TaggedTerm;
    type IntoIter = TaggedIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TaggedIterator { iter: self.statements.terms.iter() }
    }
}

impl<'a> Iterator for TaggedIterator<'a> {
    type Item = TaggedTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.iter.next()?;
        match &term {
            StatementNode::Variant(variant) => Some(TaggedTerm::Variant((**variant).clone())),
            _ => None,
        }
    }
}

impl StatementNode {
    /// Check if the statement is a variant node.
    #[inline]
    pub fn is_variant(&self) -> bool {
        matches!(self, StatementNode::Variant(_))
    }
}
