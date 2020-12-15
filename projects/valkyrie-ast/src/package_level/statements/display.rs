use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.r#type.pretty(theme)
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Nothing => ";;".into(),
            Self::Annotation(node) => node.pretty(theme),
            Self::Namespace(node) => node.pretty(theme),
            Self::Import(node) => node.pretty(theme),
            Self::Class(node) => node.pretty(theme),
            Self::ClassField(node) => node.pretty(theme),
            Self::ClassMethod(node) => node.pretty(theme),
            Self::Tagged(node) => node.pretty(theme),
            Self::Variant(node) => node.pretty(theme),
            Self::Union(node) => node.pretty(theme),
            Self::UnionField(node) => node.pretty(theme),
            Self::Enumerate(node) => node.pretty(theme),
            Self::EnumerateField(node) => node.pretty(theme),
            Self::Function(node) => node.pretty(theme),
            Self::While(node) => node.pretty(theme),
            Self::For(node) => node.pretty(theme),
            Self::Expression(node) => node.pretty(theme),
            Self::Control(node) => node.pretty(theme),
            Self::Document(node) => node.pretty(theme),
            Self::LetBind(node) => node.pretty(theme),
            Self::Guard(node) => node.pretty(theme),
            Self::Flags(node) => node.pretty(theme),
            Self::GuardLet(node) => node.pretty(theme),
        }
    }
}
