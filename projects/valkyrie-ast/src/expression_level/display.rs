use super::*;
#[cfg(feature = "lispify")]
impl Lispify for ExpressionNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        self.body.lispify()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ExpressionType {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            Self::Placeholder => Lisp::keyword("placeholder"),
            Self::Prefix(v) => v.lispify(),
            Self::Binary(v) => v.lispify(),
            Self::Suffix(v) => v.lispify(),
            Self::Tuple(v) => v.lispify(),
            Self::Array(v) => v.lispify(),
            Self::ApplyCall(v) => Lisp::keyword(format!("{v:#?}")),
            Self::SubscriptCall(v) => Lisp::keyword(format!("{v:#?}")),
            Self::GenericCall(v) => Lisp::keyword(format!("{v:#?}")),
            Self::LambdaCall(v) => Lisp::keyword(format!("{v:#?}")),
            Self::New(v) => Lisp::keyword(format!("{v:#?}")),
            Self::Resume(v) => Lisp::keyword(format!("{v:#?}")),
            Self::If(v) => v.lispify(),
            Self::IfLet(v) => Lisp::keyword(format!("{v:#?}")),
            Self::Slot(v) => Lisp::keyword(format!("{v:#?}")),
            Self::Switch(v) => Lisp::keyword(format!("{v:#?}")),
            Self::Text(v) => Lisp::string(v.text.clone()),
            Self::Try(v) => Lisp::keyword(format!("{v:#?}")),
            Self::MatchDot(v) => Lisp::keyword(format!("{v:#?}")),
            Self::Formatted(v) => Lisp::keyword(format!("{v:#?}")),
            Self::Null(v) => v.lispify(),
            Self::Boolean(v) => v.lispify(),
            Self::Symbol(v) => v.lispify(),
            Self::Number(v) => v.lispify(),
            Self::String(v) => v.lispify(),
            Self::OutputReference(v) => v.lispify(),
        }
    }
}
