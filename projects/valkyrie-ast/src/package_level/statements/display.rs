use super::*;
use lispify::{Lisp, Lispify};

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
            Self::GuardLet(node) => node.pretty(theme),
            Self::Flags(node) => node.pretty(theme),
        }
    }
}

impl Lispify for StatementNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        self.r#type.lispify()
    }
}

impl Lispify for StatementType {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            Self::Nothing => Lisp::default(),
            Self::Namespace(v) => v.lispify(),
            Self::Import(v) => todo!(),
            Self::While(v) => todo!(),
            Self::For(v) => todo!(),
            Self::Class(v) => v.lispify(),
            Self::ClassField(v) => todo!(),
            Self::ClassMethod(v) => todo!(),
            Self::Expression(v) => todo!(),
            Self::Function(v) => todo!(),
            Self::Control(v) => todo!(),
            Self::Document(v) => todo!(),
            Self::LetBind(v) => todo!(),
            Self::Guard(v) => todo!(),
            Self::Flags(v) => v.lispify(),
            Self::EnumerateField(v) => todo!(),
            Self::Tagged(v) => todo!(),
            Self::Variant(v) => todo!(),
            Self::Union(v) => v.lispify(),
            Self::Enumerate(v) => todo!(),
            Self::UnionField(v) => todo!(),
            Self::Annotation(v) => todo!(),
            Self::GuardLet(v) => todo!(),
        }
    }
}
