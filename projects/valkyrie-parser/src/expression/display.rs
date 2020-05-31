use super::*;

impl Lispify for ValkyrieExpression {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieExpression::Placeholder => Lisp::Keyword("placeholder".into()),
            ValkyrieExpression::Prefix(v) => v.as_lisp().into(),
            ValkyrieExpression::Binary(v) => v.as_lisp().into(),
            ValkyrieExpression::Suffix(v) => v.as_lisp().into(),
            ValkyrieExpression::Number(v) => v.as_lisp().into(),
            ValkyrieExpression::Symbol(v) => v.as_lisp().into(),
            ValkyrieExpression::String(v) => v.as_lisp().into(),
            ValkyrieExpression::Table(v) => v.as_lisp().into(),
        }
    }
}
