use super::*;

impl Display for ExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.body, f)
    }
}

impl IndentDisplay for ExpressionBody {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        match self {
            ExpressionBody::Placeholder => f.write_str("???"),
            ExpressionBody::Symbol(node) => f.write_str("???"),
            ExpressionBody::Number(node) => node.indent_fmt(f),
            ExpressionBody::String(node) => node.indent_fmt(f),
            ExpressionBody::Prefix(node) => node.indent_fmt(f),
            ExpressionBody::Binary(node) => node.indent_fmt(f),
            ExpressionBody::Suffix(node) => node.indent_fmt(f),
            ExpressionBody::Table(node) => node.indent_fmt(f),
            ExpressionBody::Apply(node) => node.indent_fmt(f),
            ExpressionBody::ApplyDot(node) => node.indent_fmt(f),
            ExpressionBody::LambdaCall(node) => node.indent_fmt(f),
            ExpressionBody::LambdaDot(node) => node.indent_fmt(f),
            ExpressionBody::View(node) => node.indent_fmt(f),
            ExpressionBody::GenericCall(node) => node.indent_fmt(f),
        }
    }
}

impl Display for ExpressionBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
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
