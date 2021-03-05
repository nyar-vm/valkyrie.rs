use super::*;

impl PrettyPrint for OperatorNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.operator(self.kind.as_str())
    }
}

impl PrettyPrint for UnaryNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.operator.pretty(theme).append(self.base.pretty(theme))
    }
}

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

impl PrettyPrint for PostfixNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.base.pretty(theme).append(self.operator.pretty(theme))
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

#[cfg(feature = "lispify")]
impl Lispify for PostfixNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::operator(self.operator.kind.as_str(), vec![self.base.lispify()])
    }
}
