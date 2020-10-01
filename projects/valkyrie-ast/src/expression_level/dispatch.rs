use super::*;

impl Default for ExpressionBody {
    fn default() -> Self {
        Self::Placeholder
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.body.build(theme)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionBody {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            ExpressionBody::Placeholder => unreachable!(),
            ExpressionBody::Slot(node) => node.build(theme),
            ExpressionBody::Symbol(node) => node.build(theme),
            ExpressionBody::Number(node) => node.build(theme),
            ExpressionBody::Text(node) => node.build(theme),
            ExpressionBody::String(node) => node.build(theme),
            ExpressionBody::Prefix(node) => node.build(theme),
            ExpressionBody::Binary(node) => node.build(theme),
            ExpressionBody::Suffix(node) => node.build(theme),
            ExpressionBody::Table(node) => node.build(theme),
            ExpressionBody::Apply(node) => node.build(theme),
            ExpressionBody::ApplyDot(node) => node.build(theme),
            ExpressionBody::LambdaCall(node) => node.build(theme),
            ExpressionBody::LambdaDot(node) => node.build(theme),
            ExpressionBody::Subscript(node) => node.build(theme),
            ExpressionBody::GenericCall(node) => node.build(theme),
            ExpressionBody::New(node) => node.build(theme),
            ExpressionBody::Resume(node) => node.build(theme),
            ExpressionBody::If(node) => node.build(theme),
            ExpressionBody::Switch(node) => node.build(theme),
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
