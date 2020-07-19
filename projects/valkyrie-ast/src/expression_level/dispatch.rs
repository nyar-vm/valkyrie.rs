use super::*;

impl<const T: ExpressionContext> Display for ExpressionNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.body, f)
    }
}

impl Display for ExpressionBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ExpressionBody::Placeholder => f.write_str("???"),
            ExpressionBody::Prefix(v) => Display::fmt(v, f),
            ExpressionBody::Binary(v) => Display::fmt(v, f),
            ExpressionBody::Suffix(v) => Display::fmt(v, f),
            ExpressionBody::Number(v) => Display::fmt(v, f),
            ExpressionBody::Symbol(v) => Display::fmt(v, f),
            ExpressionBody::String(v) => Display::fmt(v, f),
            ExpressionBody::Table(v) => Display::fmt(v, f),
            ExpressionBody::Apply(v) => Display::fmt(v, f),
            ExpressionBody::ApplyDot(v) => Display::fmt(v, f),
            ExpressionBody::View(v) => Display::fmt(v, f),
            ExpressionBody::GenericCall(v) => Display::fmt(v, f),
        }
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
