use super::*;
use crate::PrettyTree;

impl PrettyPrint for ExpressionNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.body.build(allocator)
    }
}

impl PrettyPrint for ExpressionBody {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            ExpressionBody::Placeholder => allocator.text("???"),
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
            ExpressionBody::View(node) => node.build(allocator),
            ExpressionBody::GenericCall(node) => node.build(allocator),
        }
    }
}

impl Default for ExpressionType {
    fn default() -> Self {
        Self::Term
    }
}

impl Default for ExpressionBody {
    fn default() -> Self {
        Self::Placeholder
    }
}

impl From<PrefixNode<ExpressionBody>> for ExpressionBody {
    fn from(value: PrefixNode<ExpressionBody>) -> Self {
        ExpressionBody::Prefix(Box::new(value))
    }
}

impl From<InfixNode<ExpressionBody>> for ExpressionBody {
    fn from(value: InfixNode<ExpressionBody>) -> Self {
        ExpressionBody::Binary(Box::new(value))
    }
}

impl From<TableNode<ExpressionBody>> for ExpressionBody {
    fn from(value: TableNode<ExpressionBody>) -> Self {
        ExpressionBody::Table(Box::new(value))
    }
}

impl From<StringLiteralNode> for ExpressionBody {
    fn from(value: StringLiteralNode) -> Self {
        ExpressionBody::String(Box::new(value))
    }
}

impl From<NumberLiteralNode> for ExpressionBody {
    fn from(value: NumberLiteralNode) -> Self {
        ExpressionBody::Number(Box::new(value))
    }
}

impl From<NamePathNode> for ExpressionBody {
    fn from(value: NamePathNode) -> Self {
        ExpressionBody::Symbol(Box::new(value))
    }
}
