use super::*;
use crate::helper::WrapDisplay;

impl Debug for BinaryNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InfixNode")
            .field("infix", &WrapDisplay::new(&self.infix.kind))
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .finish()
    }
}
impl Debug for UnaryNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UnaryNode").field("prefix", &WrapDisplay::new(&self.operator.kind)).field("base", &self.base).finish()
    }
}
impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for OperatorNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.operator(self.kind.as_str())
    }
}
#[cfg(feature = "pretty-print")]

impl PrettyPrint for UnaryNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.operator.pretty(theme).append(self.base.pretty(theme))
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for BinaryNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(5);
        items.push(self.lhs.pretty(theme));
        items.push(" ");
        items.push(self.infix.pretty(theme));
        items.push(" ");
        items.push(self.rhs.pretty(theme));
        items.into()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for UnaryNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.kind.as_str(), vec![self.base.lispify()])
    }
}
#[cfg(feature = "lispify")]
impl Lispify for BinaryNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.infix.kind.as_str(), vec![self.lhs.lispify(), self.rhs.lispify()])
    }
}
