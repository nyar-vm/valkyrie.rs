use super::*;

impl PrettyPrint for TraitDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword("trait")
    }
}
#[cfg(feature = "lispify")]
impl Lispify for TraitDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::keyword("trait")
    }
}
