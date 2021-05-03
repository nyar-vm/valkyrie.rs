use super::*;
use crate::helper::ValkyrieNode;

impl Default for ExpressionType {
    fn default() -> Self {
        Self::Placeholder
    }
}

impl Debug for PostfixCallPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Apply(call) => Debug::fmt(call, f),
            Self::View(call) => Debug::fmt(call, f),
            Self::Generic(call) => Debug::fmt(call, f),
            Self::Lambda(call) => Debug::fmt(call, f),
            Self::Match(call) => Debug::fmt(call, f),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.body.pretty(theme)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Placeholder => unreachable!(),
            Self::Slot(node) => node.pretty(theme),
            Self::Symbol(node) => node.pretty(theme),
            Self::Number(node) => node.pretty(theme),
            Self::Text(node) => node.pretty(theme),
            Self::String(node) => node.pretty(theme),
            Self::Unary(node) => node.pretty(theme),
            Self::Infix(node) => node.pretty(theme),
            Self::Tuple(node) => node.pretty(theme),
            Self::ApplyCall(node) => node.pretty(theme),
            Self::LambdaCall(node) => node.pretty(theme),
            Self::SubscriptCall(node) => node.pretty(theme),
            Self::GenericCall(node) => node.pretty(theme),
            Self::New(node) => node.pretty(theme),
            Self::Resume(node) => node.pretty(theme),
            Self::If(node) => node.pretty(theme),
            Self::IfLet(node) => node.pretty(theme),
            Self::Switch(node) => node.pretty(theme),
            Self::Try(node) => node.pretty(theme),
            Self::DotMatchCall(node) => node.pretty(theme),
            Self::Formatted(node) => node.pretty(theme),
            Self::Null(node) => node.pretty(theme),
            Self::Boolean(node) => node.pretty(theme),
            Self::OutputReference(node) => node.pretty(theme),
            Self::Array(node) => node.pretty(theme),
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PostfixCallPart {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Apply(node) => node.pretty(theme),
            Self::View(node) => node.pretty(theme),
            Self::Generic(node) => node.pretty(theme),
            Self::Lambda(node) => node.pretty(theme),
            Self::Match(node) => node.pretty(theme),
        }
    }
}

impl ValkyrieNode for ExpressionType {
    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Placeholder => unreachable!(),
            Self::Null(node) => node.get_range(),
            Self::Boolean(node) => node.get_range(),
            Self::Slot(node) => node.get_range(),
            Self::Symbol(node) => node.get_range(),
            Self::Number(node) => node.get_range(),
            Self::Text(node) => node.get_range(),
            Self::String(node) => node.get_range(),
            Self::New(node) => node.get_range(),
            Self::Object(node) => node.get_range(),
            Self::Unary(node) => node.get_range(),
            Self::Infix(node) => node.get_range(),
            Self::Tuple(node) => node.get_range(),
            Self::ApplyCall(node) => node.get_range(),
            Self::LambdaCall(node) => node.get_range(),
            Self::SubscriptCall(node) => node.get_range(),
            Self::GenericCall(node) => node.get_range(),
            Self::Resume(node) => node.get_range(),
            Self::If(node) => node.get_range(),
            Self::IfLet(node) => node.get_range(),
            Self::Switch(node) => node.get_range(),
            Self::Try(node) => node.get_range(),
            Self::Match(node) => node.get_range(),
            Self::DotMatchCall(node) => node.get_range(),
            Self::While(node) => node.get_range(),
            Self::For(node) => node.get_range(),
            Self::Formatted(node) => node.get_range(),
            Self::OutputReference(node) => node.get_range(),
            Self::Array(node) => node.get_range(),
            Self::DotCall(node) => node.get_range(),
        }
    }
}
