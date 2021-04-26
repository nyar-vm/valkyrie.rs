use super::*;
use core::fmt::Debug;

impl Debug for StatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Nothing => f.write_str("Statement::Nothing"),
            Self::Document(v) => Debug::fmt(v, f),
            Self::Annotation(v) => Debug::fmt(v, f),
            Self::Namespace(v) => Debug::fmt(v, f),
            Self::Import(v) => Debug::fmt(v, f),
            Self::Class(v) => Debug::fmt(v, f),
            Self::Union(v) => Debug::fmt(v, f),
            Self::UnionField(v) => Debug::fmt(v, f),
            Self::Enumerate(v) => Debug::fmt(v, f),
            Self::EnumerateField(v) => Debug::fmt(v, f),
            Self::Tagged(v) => Debug::fmt(v, f),
            Self::Variant(v) => Debug::fmt(v, f),
            Self::Trait(v) => Debug::fmt(v, f),
            Self::Extends(v) => Debug::fmt(v, f),
            Self::Function(v) => Debug::fmt(v, f),
            Self::While(v) => Debug::fmt(v, f),
            Self::For(v) => Debug::fmt(v, f),
            Self::Variable(v) => Debug::fmt(v, f),
            Self::Guard(v) => Debug::fmt(v, f),
            Self::Control(v) => Debug::fmt(v, f),
            Self::Expression(v) => Debug::fmt(v, f),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Nothing => ";;".into(),
            Self::Annotation(node) => node.pretty(theme),
            Self::Namespace(node) => node.pretty(theme),
            Self::Import(node) => node.pretty(theme),
            Self::Class(node) => node.pretty(theme),
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
            Self::Variable(node) => node.pretty(theme),
            Self::Guard(node) => node.pretty(theme),
            Self::Flags(node) => node.pretty(theme),
            Self::Trait(node) => node.pretty(theme),
            Self::Extends(node) => node.pretty(theme),
        }
    }
}

#[cfg(feature = "lispify")]
impl Lispify for StatementNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            Self::Nothing => Lisp::default(),
            Self::Namespace(v) => v.lispify(),
            Self::Import(v) => todo!(),
            Self::While(v) => v.lispify(),
            Self::For(v) => v.lispify(),
            Self::Class(v) => v.lispify(),
            Self::Expression(v) => v.lispify(),
            Self::Function(v) => v.lispify(),
            Self::Control(v) => todo!(),
            Self::Document(v) => todo!(),
            Self::Variable(v) => todo!(),
            Self::Guard(v) => v.lispify(),
            Self::Flags(v) => v.lispify(),
            Self::EnumerateField(v) => todo!(),
            Self::Tagged(v) => todo!(),
            Self::Variant(v) => todo!(),
            Self::Union(v) => v.lispify(),
            Self::Enumerate(v) => todo!(),
            Self::UnionField(v) => todo!(),
            Self::Annotation(v) => todo!(),
            Self::Trait(v) => v.lispify(),
            Self::Extends(v) => v.lispify(),
        }
    }
}
