use super::*;
use std::fmt::Write;
use lispify::{Lisp, Lispify, LispNumber, LispOperator};

impl Display for ValkyrieOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieOperatorKind::Not => f.write_char('!'),
            ValkyrieOperatorKind::Positive => f.write_char('+'),
            ValkyrieOperatorKind::Negative => f.write_char('-'),
            ValkyrieOperatorKind::Plus => f.write_char('+'),
            ValkyrieOperatorKind::Minus => f.write_char('-'),
            ValkyrieOperatorKind::Mul => f.write_char('*'),
            ValkyrieOperatorKind::Div => f.write_char('/'),
            ValkyrieOperatorKind::Pow => f.write_char('^'),
            ValkyrieOperatorKind::Eq => f.write_char('='),
            ValkyrieOperatorKind::Unwrap => f.write_char('!'),
            ValkyrieOperatorKind::Raise => f.write_char('?'),
            ValkyrieOperatorKind::Celsius => f.write_char('℃'),
            ValkyrieOperatorKind::Fahrenheit => f.write_char('℉'),
            ValkyrieOperatorKind::Transpose => f.write_char('ᵀ'),
            ValkyrieOperatorKind::Transjugate => f.write_char('ᴴ'),
            ValkyrieOperatorKind::Hermitian => f.write_str("Hermitian"),
        }
    }
}

impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Lispify for ValkyrieOperatorKind {
    type Output = LispOperator;

    fn lispify(&self) -> Self::Output {
        LispOperator {
            operator:  self.to_string(),
            rest: None,
        }

    }
}

impl Lispify for ValkyrieOperator {
    type Output = LispOperator;

    fn lispify(&self) -> Self::Output {
        self.kind.lispify()
    }
}

impl Lispify for ValkyrieExpression {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieExpression::Prefix(_) => {todo!()}
            ValkyrieExpression::Binary(_) => {todo!()}
            ValkyrieExpression::Suffix(_) => {todo!()}
            ValkyrieExpression::Number(_) => {todo!()}
            ValkyrieExpression::Symbol(_) => {todo!()}
        }
    }
}