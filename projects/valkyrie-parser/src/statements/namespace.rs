use super::*;

impl crate::DefineNamespaceNode {
    pub fn build(&self, ctx: &ProgramContext) -> NamespaceDeclaration {
        let kind = match &self.op_namespace {
            None => NamespaceKind::Unique,
            Some(s) => match s {
                OpNamespaceNode::Hide => NamespaceKind::Unique,
                OpNamespaceNode::Main => NamespaceKind::Shared,
                OpNamespaceNode::Test => NamespaceKind::Test,
            },
        };
        NamespaceDeclaration { kind, path: self.namepath_free.build(ctx), span: self.span.clone() }
    }
}
