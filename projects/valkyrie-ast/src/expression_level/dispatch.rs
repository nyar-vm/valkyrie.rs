use super::*;

impl Display for TermExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.expression, f)?;
        if self.eos {
            f.write_char(';')?;
        }
        Ok(())
    }
}

impl Display for TermExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TermExpressionType::Placeholder => f.write_str("???"),
            TermExpressionType::Prefix(v) => Display::fmt(v, f),
            TermExpressionType::Binary(v) => Display::fmt(v, f),
            TermExpressionType::Suffix(v) => Display::fmt(v, f),
            TermExpressionType::Number(v) => Display::fmt(v, f),
            TermExpressionType::Symbol(v) => Display::fmt(v, f),
            TermExpressionType::String(v) => Display::fmt(v, f),
            TermExpressionType::Table(v) => Display::fmt(v, f),
            TermExpressionType::Apply(v) => Display::fmt(v, f),
            TermExpressionType::ApplyDot(v) => Display::fmt(v, f),
            TermExpressionType::View(v) => Display::fmt(v, f),
            TermExpressionType::GenericCall(v) => Display::fmt(v, f),
        }
    }
}

impl From<PrefixNode<TermExpressionType>> for TermExpressionType {
    fn from(value: PrefixNode<TermExpressionType>) -> Self {
        TermExpressionType::Prefix(Box::new(value))
    }
}

impl From<InfixNode<TermExpressionType>> for TermExpressionType {
    fn from(value: InfixNode<TermExpressionType>) -> Self {
        TermExpressionType::Binary(Box::new(value))
    }
}
impl From<TableNode<TermExpressionType>> for TermExpressionType {
    fn from(value: TableNode<TermExpressionType>) -> Self {
        TermExpressionType::Table(Box::new(value))
    }
}

impl From<StringLiteralNode> for TermExpressionType {
    fn from(value: StringLiteralNode) -> Self {
        TermExpressionType::String(Box::new(value))
    }
}

impl From<NumberLiteralNode> for TermExpressionType {
    fn from(value: NumberLiteralNode) -> Self {
        TermExpressionType::Number(Box::new(value))
    }
}

impl From<NamePathNode> for TermExpressionType {
    fn from(value: NamePathNode) -> Self {
        TermExpressionType::Symbol(Box::new(value))
    }
}

impl TermExpressionType {
    pub fn update_range(&mut self) {
        match self {
            TermExpressionType::Prefix(u) => {
                let start = u.operator.range.start;
                let end = u.body.get_range().end;
                u.range = start..end;
            }
            TermExpressionType::Binary(b) => {
                let start = b.lhs.get_range().start;
                let end = b.rhs.get_range().end;
                b.range = start..end;
            }
            TermExpressionType::Suffix(u) => {
                let start = u.body.get_range().start;
                let end = u.operator.range.end;
                u.range = start..end;
            }
            _ => {}
        }
    }
}
