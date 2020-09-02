use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for NamespaceDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = allocator.keyword(self.kind.as_str());
        let space = allocator.space();
        let path = allocator.join(&self.path, ".");
        let semi = allocator.text(";");
        head.append(space).append(path).append(semi)
    }
}

impl NamespaceKind {
    /// Get the string representation of the namespace kind
    pub fn as_str(&self) -> &'static str {
        match self {
            NamespaceKind::Shared => "namespace",
            NamespaceKind::Unique => "namespace!",
            NamespaceKind::Test => "namespace?",
        }
    }
}
