use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.r#type.build(theme)
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementBody {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Nothing => theme.text(";;"),
            Self::Annotation(node) => node.build(theme),
            Self::Namespace(node) => node.build(theme),
            Self::Import(node) => node.build(theme),
            Self::Class(node) => node.build(theme),
            Self::ClassField(node) => node.build(theme),
            Self::ClassMethod(node) => node.build(theme),
            Self::Tagged(node) => node.build(theme),
            Self::Variant(node) => node.build(theme),
            Self::Union(node) => node.build(theme),
            Self::UnionField(node) => node.build(theme),
            Self::Enumerate(node) => node.build(theme),
            Self::EnumerateField(node) => node.build(theme),
            Self::Function(node) => node.build(theme),
            Self::While(node) => node.build(theme),
            Self::For(node) => node.build(theme),
            Self::Expression(node) => node.build(theme),
            Self::Control(node) => node.build(theme),
            Self::Document(node) => node.build(theme),
            Self::LetBind(node) => node.build(theme),
            Self::Guard(node) => node.build(theme),
            Self::Flags(node) => node.build(theme),
        }
    }
}
