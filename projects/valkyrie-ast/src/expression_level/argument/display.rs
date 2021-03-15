use super::*;

impl Default for ApplyCaller {
    fn default() -> Self {
        Self::None
    }
}

impl PrettyPrint for ApplyCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        if let Some(s) = &self.arguments {
            terms += s.pretty(theme)
        }
        terms.into()
    }
}

impl PrettyPrint for ApplyCallTerms {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += "(";
        terms += theme.join(self.terms.clone(), ", ");
        terms += ")";
        terms.into()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for ApplyCallNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("argument");
        match &self.arguments {
            Some(s) => {
                lisp += s.lispify();
            }
            None => {}
        }

        lisp
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ApplyCallTerms {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::keyword("ApplyCallTerms: ???")
    }
}
impl PrettyPrint for ApplyCallItem {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.text("ApplyCallTerm ???")
    }
}

#[cfg(feature = "lispify")]
impl Lispify for ApplyCallItem {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::keyword("ApplyCallTerm: ???")
    }
}

impl PrettyPrint for ArgumentNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        PrettyTree::text("(").append(theme.join(self.terms.clone(), ", ")).append(")")
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ArgumentNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(self.terms.len());
        for term in self.terms.iter() {
            lisp += term.lispify();
        }
        lisp
    }
}

impl PrettyPrint for ArgumentTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.term.pretty(theme)
    }
}

#[cfg(feature = "lispify")]
impl Lispify for ArgumentTerm {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}

impl PrettyPrint for ArgumentKey {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mods = self.modifiers.pretty(theme);
        let key = theme.argument(self.key.name.clone(), false);
        mods.append(key)
    }
}
