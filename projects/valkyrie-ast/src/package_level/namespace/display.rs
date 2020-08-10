use super::*;

impl PrettyPrint for NamespaceDeclarationNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = allocator.keyword(self.kind.as_str());
        let space = allocator.space();
        let path = allocator.join(&self.path, ".");
        let semi = allocator.text(";");
        head.append(space).append(path).append(semi)
    }
}
