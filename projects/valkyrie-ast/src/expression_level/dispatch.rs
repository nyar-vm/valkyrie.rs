use super::*;

impl Default for ExpressionBody {
    fn default() -> Self {
        Self::Placeholder
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.body.build(allocator)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExpressionBody {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            ExpressionBody::Placeholder => unreachable!(),
            ExpressionBody::Slot(node) => node.build(allocator),
            ExpressionBody::Symbol(node) => node.build(allocator),
            ExpressionBody::Number(node) => node.build(allocator),
            ExpressionBody::String(node) => node.build(allocator),
            ExpressionBody::Prefix(node) => node.build(allocator),
            ExpressionBody::Binary(node) => node.build(allocator),
            ExpressionBody::Suffix(node) => node.build(allocator),
            ExpressionBody::Table(node) => node.build(allocator),
            ExpressionBody::Apply(node) => node.build(allocator),
            ExpressionBody::ApplyDot(node) => node.build(allocator),
            ExpressionBody::LambdaCall(node) => node.build(allocator),
            ExpressionBody::LambdaDot(node) => node.build(allocator),
            ExpressionBody::Subscript(node) => node.build(allocator),
            ExpressionBody::GenericCall(node) => node.build(allocator),
            ExpressionBody::New(node) => node.build(allocator),
            ExpressionBody::Control(node) => node.build(allocator),
            ExpressionBody::If(node) => node.build(allocator),
            ExpressionBody::Switch(node) => node.build(allocator),
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
            ExpressionBody::Control(node) => node.span.clone(),
            ExpressionBody::If(node) => node.span.clone(),
            ExpressionBody::Switch(node) => node.span.clone(),
        }
    }
}
