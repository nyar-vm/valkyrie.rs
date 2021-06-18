use super::*;

impl Debug for TraitDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut match &self.kind {
            TraitKind::Trait => f.debug_struct("Trait"),
            TraitKind::Interface => f.debug_struct("Interface"),
        };
        w.field("name", &self.name);
        w.field("terms", &self.terms);
        w.finish()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for TraitDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword("trait")
    }
}

#[cfg(feature = "lispify")]
impl Lispify for TraitDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut out = Lisp::new(10);
        out += Lisp::keyword("trait");
        out += Lisp::string(self.name.to_string());
        for field in &self.terms {
            let mut inside = Lisp::new(10);
            inside += Lisp::keyword("trait/field");
            inside += Lisp::string(field.name.to_string());
            out += inside;
        }
        for field in &self.methods {
            let mut inside = Lisp::new(10);
            inside += Lisp::keyword("trait/methods");
            inside += Lisp::string(field.method_name.to_string());
            out += inside;
        }
        out
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ExtendsStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword("extends")
    }
}

#[cfg(feature = "lispify")]
impl Lispify for ExtendsStatement {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::keyword("extends")
    }
}

impl Debug for TraitTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Macro(v) => Debug::fmt(v, f),
            Self::Field(v) => Debug::fmt(v, f),
            Self::Method(v) => Debug::fmt(v, f),
        }
    }
}
