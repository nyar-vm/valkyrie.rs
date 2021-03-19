use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ApplyCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        if let Some(s) = &self.arguments {
            terms += s.pretty(theme)
        }
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

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ArgumentsList {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        PrettyTree::text("(").append(theme.join(self.terms.clone(), ", ")).append(")")
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ArgumentsList {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(self.terms.len());
        for term in self.terms.iter() {
            lisp += term.lispify();
        }
        lisp
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ArgumentTerm {
    fn pretty(&self, _: &PrettyProvider) -> PrettyTree {
        todo!()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for ArgumentTerm {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ArgumentKey {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mods = self.modifiers.pretty(theme);
        let key = theme.argument(self.key.name.clone(), false);
        mods.append(key)
    }
}
