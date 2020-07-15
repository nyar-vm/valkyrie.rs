use super::*;

impl<T: Display> Display for ExpressionNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.expression, f)
    }
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ExpressionType::Placeholder => f.write_str("???"),
            ExpressionType::Prefix(v) => Display::fmt(v, f),
            ExpressionType::Binary(v) => Display::fmt(v, f),
            ExpressionType::Suffix(v) => Display::fmt(v, f),
            ExpressionType::Number(v) => Display::fmt(v, f),
            ExpressionType::Symbol(v) => Display::fmt(v, f),
            ExpressionType::String(v) => Display::fmt(v, f),
            ExpressionType::Table(v) => Display::fmt(v, f),
            ExpressionType::Apply(v) => Display::fmt(v, f),
            ExpressionType::ApplyDot(v) => Display::fmt(v, f),
            ExpressionType::View(v) => Display::fmt(v, f),
            ExpressionType::GenericCall(v) => Display::fmt(v, f),
        }
    }
}

impl From<PrefixNode<ExpressionType>> for ExpressionType {
    fn from(value: PrefixNode<ExpressionType>) -> Self {
        ExpressionType::Prefix(Box::new(value))
    }
}

impl From<InfixNode<ExpressionType>> for ExpressionType {
    fn from(value: InfixNode<ExpressionType>) -> Self {
        ExpressionType::Binary(Box::new(value))
    }
}
impl From<TableNode<ExpressionType>> for ExpressionType {
    fn from(value: TableNode<ExpressionType>) -> Self {
        ExpressionType::Table(Box::new(value))
    }
}

impl From<StringLiteralNode> for ExpressionType {
    fn from(value: StringLiteralNode) -> Self {
        ExpressionType::String(Box::new(value))
    }
}

impl From<NumberLiteralNode> for ExpressionType {
    fn from(value: NumberLiteralNode) -> Self {
        ExpressionType::Number(Box::new(value))
    }
}

impl From<NamePathNode> for ExpressionType {
    fn from(value: NamePathNode) -> Self {
        ExpressionType::Symbol(Box::new(value))
    }
}

impl ExpressionType {
    pub fn update_range(&mut self) {
        match self {
            ExpressionType::Prefix(u) => {
                let start = u.operator.range.start;
                let end = u.body.get_range().end;
                u.range = start..end;
            }
            ExpressionType::Binary(b) => {
                let start = b.lhs.get_range().start;
                let end = b.rhs.get_range().end;
                b.range = start..end;
            }
            ExpressionType::Suffix(u) => {
                let start = u.body.get_range().start;
                let end = u.operator.range.end;
                u.range = start..end;
            }
            _ => {}
        }
    }
}
