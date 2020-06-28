use super::*;

impl Display for TermExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TermExpressionNode::Placeholder => f.write_str("???"),
            TermExpressionNode::Prefix(v) => Display::fmt(v, f),
            TermExpressionNode::Binary(v) => Display::fmt(v, f),
            TermExpressionNode::Suffix(v) => Display::fmt(v, f),
            TermExpressionNode::Number(v) => Display::fmt(v, f),
            TermExpressionNode::Symbol(v) => Display::fmt(v, f),
            TermExpressionNode::String(v) => Display::fmt(v, f),
            TermExpressionNode::Table(v) => Display::fmt(v, f),
            TermExpressionNode::Apply(v) => Display::fmt(v, f),
            TermExpressionNode::ApplyDot(v) => Display::fmt(v, f),
            TermExpressionNode::View(v) => Display::fmt(v, f),
            TermExpressionNode::GenericCall(v) => Display::fmt(v, f),
        }
    }
}

impl From<PrefixNode<TermExpressionNode>> for TermExpressionNode {
    fn from(value: PrefixNode<TermExpressionNode>) -> Self {
        TermExpressionNode::Prefix(Box::new(value))
    }
}

impl From<InfixNode<TermExpressionNode>> for TermExpressionNode {
    fn from(value: InfixNode<TermExpressionNode>) -> Self {
        TermExpressionNode::Binary(Box::new(value))
    }
}
impl From<TableNode<TermExpressionNode>> for TermExpressionNode {
    fn from(value: TableNode<TermExpressionNode>) -> Self {
        TermExpressionNode::Table(Box::new(value))
    }
}

impl From<StringLiteralNode> for TermExpressionNode {
    fn from(value: StringLiteralNode) -> Self {
        TermExpressionNode::String(Box::new(value))
    }
}

impl From<NumberLiteralNode> for TermExpressionNode {
    fn from(value: NumberLiteralNode) -> Self {
        TermExpressionNode::Number(Box::new(value))
    }
}

impl From<NamePathNode> for TermExpressionNode {
    fn from(value: NamePathNode) -> Self {
        TermExpressionNode::Symbol(Box::new(value))
    }
}

impl TermExpressionNode {
    pub fn update_range(&mut self) {
        match self {
            TermExpressionNode::Prefix(u) => {
                let start = u.operator.range.start;
                let end = u.body.get_range().end;
                u.range = start..end;
            }
            TermExpressionNode::Binary(b) => {
                let start = b.lhs.get_range().start;
                let end = b.rhs.get_range().end;
                b.range = start..end;
            }
            TermExpressionNode::Suffix(u) => {
                let start = u.body.get_range().start;
                let end = u.operator.range.end;
                u.range = start..end;
            }
            _ => {}
        }
    }
}
