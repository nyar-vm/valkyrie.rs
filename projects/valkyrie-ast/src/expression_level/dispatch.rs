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
            Self::ApplyDot(call) => Debug::fmt(call, f),
            Self::View(call) => Debug::fmt(call, f),
            Self::Generic(call) => Debug::fmt(call, f),
            Self::Lambda(call) => Debug::fmt(call, f),
            Self::LambdaDot(call) => Debug::fmt(call, f),
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
            Self::Prefix(node) => node.pretty(theme),
            Self::Binary(node) => node.pretty(theme),
            Self::Suffix(node) => node.pretty(theme),
            Self::Table(node) => node.pretty(theme),
            Self::Apply(node) => node.pretty(theme),
            Self::ApplyDot(node) => node.pretty(theme),
            Self::LambdaCall(node) => node.pretty(theme),
            Self::LambdaDot(node) => node.pretty(theme),
            Self::Subscript(node) => node.pretty(theme),
            Self::GenericCall(node) => node.pretty(theme),
            Self::New(node) => node.pretty(theme),
            Self::Resume(node) => node.pretty(theme),
            Self::If(node) => node.pretty(theme),
            Self::IfLet(node) => node.pretty(theme),
            Self::Switch(node) => node.pretty(theme),
            Self::Try(node) => node.pretty(theme),
            Self::MatchDot(node) => node.pretty(theme),
            Self::Formatted(node) => node.pretty(theme),
        }
    }
}

impl PrettyPrint for PostfixCallPart {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Apply(node) => node.pretty(theme),
            Self::ApplyDot(node) => node.pretty(theme),
            Self::View(node) => node.pretty(theme),
            Self::Generic(node) => node.pretty(theme),
            Self::Lambda(node) => node.pretty(theme),
            Self::LambdaDot(node) => node.pretty(theme),
            Self::Match(node) => node.pretty(theme),
        }
    }
}

impl ValkyrieNode for ExpressionType {
    fn get_range(&self) -> Range<u32> {
        match self {
            Self::Placeholder => unreachable!(),
            Self::Slot(node) => node.span.clone(),
            Self::Symbol(node) => node.span.clone(),
            Self::Number(node) => node.span.clone(),
            Self::Text(node) => node.span.clone(),
            Self::String(node) => node.span.clone(),
            Self::New(node) => node.span.clone(),
            Self::Prefix(node) => node.span.clone(),
            Self::Binary(node) => node.span.clone(),
            Self::Suffix(node) => node.span.clone(),
            Self::Table(node) => node.span.clone(),
            Self::Apply(node) => node.span.clone(),
            Self::ApplyDot(node) => node.span.clone(),
            Self::LambdaCall(node) => node.span.clone(),
            Self::LambdaDot(node) => node.span.clone(),
            Self::Subscript(node) => node.span.clone(),
            Self::GenericCall(node) => node.span.clone(),
            Self::Resume(node) => node.span.clone(),
            Self::If(node) => node.span.clone(),
            Self::IfLet(node) => node.span.clone(),
            Self::Switch(node) => node.span.clone(),
            Self::Try(node) => node.span.clone(),
            Self::MatchDot(node) => node.span.clone(),
            Self::Formatted(node) => node.span.clone(),
        }
    }
}
