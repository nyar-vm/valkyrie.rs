use super::*;

impl Lispify for ValkyrieExpression {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieExpression::Placeholder => Lisp::Keyword("placeholder".into()),
            ValkyrieExpression::Prefix(v) => v.lispify().into(),
            ValkyrieExpression::Binary(v) => v.lispify().into(),
            ValkyrieExpression::Suffix(v) => v.lispify().into(),
            ValkyrieExpression::Number(v) => v.as_lisp().into(),
            ValkyrieExpression::Symbol(v) => v.as_lisp().into(),
            ValkyrieExpression::String(v) => v.as_lisp().into(),
            ValkyrieExpression::Table(v) => v.as_lisp().into(),
        }
    }
}

impl Lispify for BinaryNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.to_string(), &[self.lhs.lispify(), self.rhs.lispify()])
    }
}
impl Lispify for UnaryNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.to_string(), &[self.body.lispify()])
    }
}
