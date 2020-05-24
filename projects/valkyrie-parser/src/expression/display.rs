use super::*;
use valkyrie_ast::ValkyrieIdentifier;

impl Lispify for ValkyrieExpression {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieExpression::Placeholder => Lisp::Keyword("placeholder".into()),
            ValkyrieExpression::Prefix(v) => v.lispify().into(),
            ValkyrieExpression::Binary(v) => v.lispify().into(),
            ValkyrieExpression::Suffix(v) => v.lispify().into(),
            ValkyrieExpression::Number(v) => v.lispify().into(),
            ValkyrieExpression::Symbol(v) => v.lispify().into(),
            ValkyrieExpression::String(v) => v.lispify().into(),
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

impl Lispify for ValkyrieBinary {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.to_string(), &[self.lhs.lispify(), self.rhs.lispify()])
    }
}
impl Lispify for ValkyrieUnary {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.to_string(), &[self.body.lispify()])
    }
}
