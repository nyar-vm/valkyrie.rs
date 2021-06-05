use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for UnionDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("union");
        terms += " ";
        terms += self.name.pretty(theme);
        terms += self.terms.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for UnionDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("union");
        lisp += self.name.lispify();
        lisp
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for VariantDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += self.modifiers.pretty(theme);
        terms += theme.argument(self.name.name.to_string(), false);
        terms += theme.keyword(":");
        terms += " ";
        terms.into()
    }
}

impl Debug for UnionTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Macro(v) => Debug::fmt(v, f),
            Self::Variant(v) => Debug::fmt(v, f),
            Self::Method(v) => Debug::fmt(v, f),
        }
    }
}

#[cfg(feature = "lispify")]
impl Lispify for VariantDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
