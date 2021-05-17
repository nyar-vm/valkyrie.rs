use super::*;

impl crate::DefineNamespaceNode {
    pub fn build(&self, ctx: &mut ProgramState) -> NamespaceDeclaration {
        let kind = match &self.op_namespace {
            None => NamespaceKind::Standalone,
            Some(s) => match s {
                OpNamespaceNode::Hide => NamespaceKind::Standalone,
                OpNamespaceNode::Main => NamespaceKind::Main,
                OpNamespaceNode::Test => NamespaceKind::Test,
            },
        };
        NamespaceDeclaration { kind, path: self.namepath_free.build(ctx), span: self.span.clone() }
    }
}
