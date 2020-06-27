use super::*;
use crate::traits::ThisParser;

impl Display for ValkyrieREPL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieREPL::Namespace(v) => Display::fmt(v, f),
            ValkyrieREPL::Expression(v) => Display::fmt(v, f),
        }
    }
}

impl Lispify for ValkyrieREPL {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieREPL::Expression(v) => v.as_lisp(),
            ValkyrieREPL::Namespace(v) => v.as_lisp(),
        }
    }
}
