use super::*;

impl Debug for ExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Placeholder => f.write_str("Placeholder"),
            Self::Null(v) => Debug::fmt(v, f),
            Self::Boolean(v) => Display::fmt(v, f),
            Self::Slot(v) => Debug::fmt(v, f),
            Self::Symbol(v) => Display::fmt(v, f),
            Self::Number(v) => Display::fmt(v, f),
            Self::Text(v) => Display::fmt(v, f),
            Self::String(v) => Display::fmt(v, f),
            Self::Formatted(v) => Debug::fmt(v, f),
            Self::New(v) => Debug::fmt(v, f),
            Self::Object(v) => Debug::fmt(v, f),
            Self::Unary(v) => Debug::fmt(v, f),
            Self::Infix(v) => Debug::fmt(v, f),
            Self::Tuple(v) => Debug::fmt(v, f),
            Self::Array(v) => Debug::fmt(v, f),
            Self::Resume(v) => Debug::fmt(v, f),
            Self::If(v) => Debug::fmt(v, f),
            Self::IfLet(v) => Debug::fmt(v, f),
            Self::Switch(v) => Debug::fmt(v, f),
            Self::Try(v) => Debug::fmt(v, f),
            Self::Match(v) => Debug::fmt(v, f),
            Self::ApplyCall(v) => Debug::fmt(v, f),
            Self::LambdaCall(v) => Debug::fmt(v, f),
            Self::SubscriptCall(v) => Debug::fmt(v, f),
            Self::GenericCall(v) => Debug::fmt(v, f),
            Self::DotMatchCall(v) => Debug::fmt(v, f),
            Self::OutputReference(v) => Debug::fmt(v, f),
            Self::DotCall(v) => Debug::fmt(v, f),
        }
    }
}

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
            Self::Unary(v) => v.lispify(),
            Self::Infix(v) => v.lispify(),
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
            Self::DotMatchCall(v) => Lisp::keyword(format!("{v:#?}")),
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
