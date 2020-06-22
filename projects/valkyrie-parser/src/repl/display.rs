use super::*;
use crate::traits::ThisParser;

impl Display for ValkyrieREPL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieREPL::Namespace(v) => Display::fmt(v, f),
            ValkyrieREPL::Expression(v) => Display::fmt(v, f),
        }
    }
}

impl Display for TermExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        }
    }
}

impl Lispify for ValkyrieREPL {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieREPL::Expression(v) => v.as_lisp(),
            ValkyrieREPL::Namespace(v) => v.as_lisp(),
        }
    }
}
