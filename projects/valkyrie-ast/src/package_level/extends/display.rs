use super::*;

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
