use super::*;

impl Debug for ExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Placeholder => f.write_str("Placeholder"),
            Self::Null(node) => Debug::fmt(node, f),
            Self::Boolean(node) => Display::fmt(node, f),
            Self::Symbol(node) => Display::fmt(node, f),
            Self::Number(node) => Display::fmt(node, f),
            Self::Text(node) => Display::fmt(node, f),
            Self::String(node) => Display::fmt(node, f),
            Self::Formatted(node) => Debug::fmt(node, f),
            Self::New(node) => Debug::fmt(node, f),
            Self::Object(node) => Debug::fmt(node, f),
            Self::Lambda(node) => Debug::fmt(node, f),
            Self::Slot(node) => Debug::fmt(node, f),
            Self::Unary(node) => Debug::fmt(node, f),
            Self::Infix(node) => Debug::fmt(node, f),
            Self::Tuple(node) => Debug::fmt(node, f),
            Self::Array(node) => Debug::fmt(node, f),
            Self::Resume(node) => Debug::fmt(node, f),
            Self::If(node) => Debug::fmt(node, f),
            Self::IfLet(node) => Debug::fmt(node, f),
            Self::Switch(node) => Debug::fmt(node, f),
            Self::Try(node) => Debug::fmt(node, f),
            Self::Match(node) => Debug::fmt(node, f),
            Self::ApplyCall(node) => Debug::fmt(node, f),
            Self::ClosureCall(node) => Debug::fmt(node, f),
            Self::SubscriptCall(node) => Debug::fmt(node, f),
            Self::GenericCall(node) => Debug::fmt(node, f),
            Self::DotMatchCall(node) => Debug::fmt(node, f),
            Self::OutputReference(node) => Debug::fmt(node, f),
            Self::DotCall(node) => Debug::fmt(node, f),
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
            Self::ClosureCall(v) => Lisp::keyword(format!("{v:#?}")),
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
