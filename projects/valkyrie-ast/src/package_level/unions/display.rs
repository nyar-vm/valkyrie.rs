use super::*;

impl Debug for UnionDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("Union");
        if !self.annotations.is_empty() {
            w.field("annotations", &self.annotations);
        }
        w.field("name", &WrapDisplay::new(&self.name));
        if !self.inherits.is_empty() {
            w.field("inherits", &self.inherits);
        }
        if let Some(i) = &self.implements {
            w.field("implements", i);
        }
        w.field("body", &self.body);
        w.field("span", &self.span);
        w.finish()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for UnionDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("union");
        terms += " ";
        terms += self.name.pretty(theme);
        terms += self.body.pretty(theme);
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
impl Debug for VariantDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("Variant");
        if !self.annotations.is_empty() {
            w.field("annotations", &self.annotations);
        }
        w.field("name", &WrapDisplay::new(&self.name));
        if !self.body.is_empty() {
            w.field("body", &self.body);
        }
        w.finish()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for VariantDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
