use super::*;

impl Default for ExpressionBody {
    fn default() -> Self {
        Self::Placeholder
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.body.pretty(theme)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionBody {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            ExpressionBody::Placeholder => unreachable!(),
            ExpressionBody::Slot(node) => node.pretty(theme),
            ExpressionBody::Symbol(node) => node.pretty(theme),
            ExpressionBody::Number(node) => node.pretty(theme),
            ExpressionBody::Text(node) => node.pretty(theme),
            ExpressionBody::String(node) => node.pretty(theme),
            ExpressionBody::Prefix(node) => node.pretty(theme),
            ExpressionBody::Binary(node) => node.pretty(theme),
            ExpressionBody::Suffix(node) => node.pretty(theme),
            ExpressionBody::Table(node) => node.pretty(theme),
            ExpressionBody::Apply(node) => node.pretty(theme),
            ExpressionBody::ApplyDot(node) => node.pretty(theme),
            ExpressionBody::LambdaCall(node) => node.pretty(theme),
            ExpressionBody::LambdaDot(node) => node.pretty(theme),
            ExpressionBody::Subscript(node) => node.pretty(theme),
            ExpressionBody::GenericCall(node) => node.pretty(theme),
            ExpressionBody::New(node) => node.pretty(theme),
            ExpressionBody::Resume(node) => node.pretty(theme),
            ExpressionBody::If(node) => node.pretty(theme),
            ExpressionBody::Switch(node) => node.pretty(theme),
        }
    }
}

impl ExpressionBody {
    pub fn span(&self) -> Range<u32> {
        match self {
            ExpressionBody::Placeholder => unreachable!(),
            ExpressionBody::Slot(node) => node.span.clone(),
            ExpressionBody::Symbol(node) => node.span.clone(),
            ExpressionBody::Number(node) => node.span.clone(),
            ExpressionBody::Text(node) => node.span.clone(),
            ExpressionBody::String(node) => node.span.clone(),
            ExpressionBody::New(node) => node.span.clone(),
            ExpressionBody::Prefix(node) => node.span.clone(),
            ExpressionBody::Binary(node) => node.span.clone(),
            ExpressionBody::Suffix(node) => node.span.clone(),
            ExpressionBody::Table(node) => node.span.clone(),
            ExpressionBody::Apply(node) => node.span.clone(),
            ExpressionBody::ApplyDot(node) => node.span.clone(),
            ExpressionBody::LambdaCall(node) => node.span.clone(),
            ExpressionBody::LambdaDot(node) => node.span.clone(),
            ExpressionBody::Subscript(node) => node.span.clone(),
            ExpressionBody::GenericCall(node) => node.span.clone(),
            ExpressionBody::Resume(node) => node.span.clone(),
            ExpressionBody::If(node) => node.span.clone(),
            ExpressionBody::Switch(node) => node.span.clone(),
        }
    }
}
