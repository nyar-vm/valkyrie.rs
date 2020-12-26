use super::*;
use lispify::{Lisp, Lispify};

impl Display for IdentifierNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

impl Display for NamePathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut iter = self.names.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, "∷{}", item)?;
            }
        }
        Ok(())
    }
}

impl PrettyPrint for IdentifierNode {
    fn pretty(&self, _: &PrettyProvider) -> PrettyTree {
        PrettyTree::text(self.name.to_string())
    }
}

impl Lispify for IdentifierNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::symbol(self.to_string())
    }
}

impl PrettyPrint for LambdaSlotNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword(format!("${}", self.name))
    }
}

impl PrettyPrint for NamePathNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.join(self.names.clone(), "∷")
    }
}
