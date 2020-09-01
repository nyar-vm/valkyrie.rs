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
            StatementBody::Nothing => allocator.text(";;"),
            StatementBody::Namespace(node) => node.build(allocator),
            StatementBody::Import(node) => node.build(allocator),
            StatementBody::Class(node) => node.build(allocator),
            StatementBody::Function(node) => node.build(allocator),
            StatementBody::While(node) => node.build(allocator),
            StatementBody::For(node) => node.build(allocator),
            StatementBody::Expression(node) => node.build(allocator),
            StatementBody::Control(node) => node.build(allocator),
            StatementBody::Document(node) => node.build(allocator),
            StatementBody::LetBind(node) => node.build(allocator),
            StatementBody::Guard(node) => node.build(allocator),
            StatementBody::Flags(node) => node.build(allocator),
            StatementBody::FlagsField(node) => node.build(allocator),
        }
    }
}
