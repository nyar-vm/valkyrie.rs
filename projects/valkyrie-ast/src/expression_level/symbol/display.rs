use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for IdentifierNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.text(self.name.to_string())
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for LambdaSlotNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.keyword(format!("${}", self.name))
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for NamePathNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.join(&self.names, "∷")
    }
}
