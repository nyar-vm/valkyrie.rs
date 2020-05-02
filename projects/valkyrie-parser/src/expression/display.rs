use super::*;
use crate::symbol::ValkyrieIdentifier;
use lispify::{Lisp, LispNumber, LispOperator, LispSymbol, Lispify};
use std::fmt::Write;

impl Display for ValkyrieOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieOperatorKind::Not => f.write_char('!'),
            ValkyrieOperatorKind::Concat => f.write_str("++"),
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
        LispOperator { operator: self.to_string(), rest: None }
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
            ValkyrieExpression::Prefix(v) => v.lispify().into(),
            ValkyrieExpression::Binary(v) => v.lispify().into(),
            ValkyrieExpression::Suffix(v) => v.lispify().into(),
            ValkyrieExpression::Number(v) => v.lispify().into(),
            ValkyrieExpression::Symbol(v) => v.lispify().into(),
        }
    }
}

impl Lispify for ValkyrieNamepath {
    type Output = LispSymbol;

    fn lispify(&self) -> Self::Output {
        let mut terms = self.names.iter().map(|s| s.name.clone());
        let first = terms.next().unwrap_or_default();

        LispSymbol { name: first, path: terms.collect() }
    }
}

impl Lispify for ValkyrieIdentifier {
    type Output = LispSymbol;

    fn lispify(&self) -> Self::Output {
        LispSymbol { name: self.name.clone(), path: vec![] }
    }
}

impl Lispify for ValkyrieBinary {
    type Output = LispOperator;

    fn lispify(&self) -> Self::Output {
        let head = self.operator.lispify().operator;
        let rest = vec![self.lhs.lispify(), self.rhs.lispify()];
        LispOperator { operator: head, rest: Some(rest) }
    }
}
impl Lispify for ValkyrieUnary {
    type Output = LispOperator;

    fn lispify(&self) -> Self::Output {
        let head = self.operator.lispify().operator;
        let rest = vec![self.body.lispify()];
        LispOperator { operator: head, rest: Some(rest) }
    }
}
