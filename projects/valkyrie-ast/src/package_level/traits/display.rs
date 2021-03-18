use super::*;
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
        for field in &self.fields {
            let mut inside = Lisp::new(10);
            inside += Lisp::keyword("trait/field");
            inside += Lisp::string(field.field_name.to_string());
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
