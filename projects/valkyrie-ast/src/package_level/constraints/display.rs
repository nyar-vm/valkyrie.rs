use super::*;

impl Debug for ConstraintTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Macro(v) => Debug::fmt(v, f),
            Self::Field(v) => Debug::fmt(v, f),
            Self::Method(v) => Debug::fmt(v, f),
            Self::Domain(v) => Debug::fmt(v, f),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ConstraintDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("class");
        terms += " ";
        terms += self.name.pretty(theme);
        if let Some(gen) = &self.generic {
            terms += gen.pretty(theme);
        }
        terms += " ";
        let block = SoftBlock::curly_braces().with_joint(PrettyTree::text(";").append(PrettyTree::Hardline));
        terms += block.join_slice(&self.terms, theme);
        terms += block.join_slice(&self.methods, theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ConstraintDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("define/class");
        lisp += self.name.lispify();
        lisp += self.modifiers.lispify();
        for item in &self.terms {
            lisp += item.lispify();
        }
        for item in &self.methods {
            lisp += item.lispify();
        }
        lisp
    }
}
