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
            Self::Number(v) => v.lispify(),
            Self::Symbol(v) => v.lispify(),
            Self::String(v) => todo!(),
            Self::Table(v) => todo!(),
            Self::Apply(v) => todo!(),
            Self::ApplyDot(v) => todo!(),
            Self::Subscript(v) => todo!(),
            Self::GenericCall(v) => todo!(),
            Self::LambdaCall(v) => todo!(),
            Self::LambdaDot(v) => todo!(),
            Self::New(v) => todo!(),
            Self::Resume(v) => todo!(),
            Self::If(v) => v.lispify(),
            Self::IfLet(v) => todo!(),
            Self::Slot(v) => todo!(),
            Self::Switch(v) => todo!(),
            Self::Text(v) => Lisp::string(v.text.clone()),
            Self::Try(v) => todo!(),
            Self::MatchDot(v) => todo!(),
            Self::Formatted(v) => todo!(),
            Self::Null(v) => v.lispify(),
            Self::Boolean(v) => v.lispify(),
        }
    }
}
