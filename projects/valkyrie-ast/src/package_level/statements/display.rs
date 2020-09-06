use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.r#type.build(allocator)
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementBody {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Nothing => allocator.text(";;"),
            Self::Annotation(node) => node.build(allocator),
            Self::Namespace(node) => node.build(allocator),
            Self::Import(node) => node.build(allocator),
            Self::Class(node) => node.build(allocator),
            Self::ClassField(node) => node.build(allocator),
            Self::ClassMethod(node) => node.build(allocator),
            Self::Tagged(node) => node.build(allocator),
            Self::Variant(node) => node.build(allocator),
            Self::Union(node) => node.build(allocator),
            Self::UnionField(node) => node.build(allocator),
            Self::Enumerate(node) => node.build(allocator),
            Self::EnumerateField(node) => node.build(allocator),
            Self::Function(node) => node.build(allocator),
            Self::While(node) => node.build(allocator),
            Self::For(node) => node.build(allocator),
            Self::Expression(node) => node.build(allocator),
            Self::Control(node) => node.build(allocator),
            Self::Document(node) => node.build(allocator),
            Self::LetBind(node) => node.build(allocator),
            Self::Guard(node) => node.build(allocator),
            Self::Flags(node) => node.build(allocator),
        }
    }
}
