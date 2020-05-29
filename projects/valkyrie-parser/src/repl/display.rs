use super::*;
use crate::{
    expression::{BinaryNode, UnaryNode},
    traits::ThisParser,
};

impl Display for ValkyrieREPL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieREPL::Namespace(v) => Display::fmt(v, f),
            ValkyrieREPL::Expression(v) => Display::fmt(v, f),
        }
    }
}

impl Display for ValkyrieExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieExpression::Placeholder => f.write_str("???"),
            ValkyrieExpression::Prefix(v) => Display::fmt(v, f),
            ValkyrieExpression::Binary(v) => Display::fmt(v, f),
            ValkyrieExpression::Suffix(v) => Display::fmt(v, f),
            ValkyrieExpression::Number(v) => Display::fmt(v, f),
            ValkyrieExpression::Symbol(v) => Display::fmt(v, f),
            ValkyrieExpression::String(v) => Display::fmt(v, f),
            ValkyrieExpression::Table(v) => Display::fmt(v, f),
        }
    }
}

impl Display for UnaryNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.operator, self.body)
    }
}

impl Display for BinaryNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.operator, self.rhs)
    }
}

impl Lispify for ValkyrieREPL {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieREPL::Expression(v) => v.lispify(),
            ValkyrieREPL::Namespace(v) => v.as_lisp(),
        }
    }
}
