use super::*;

impl Debug for NamespaceDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NamespaceDeclaration").field("kind", &self.kind).field("path", &WrapDisplay::new(&self.path)).finish()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for NamespaceDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let head = theme.keyword(self.kind.as_str());
        let space = " ";
        let path = theme.join(self.path.path.clone(), ".");
        let semi = ";";
        head.append(space).append(path).append(semi)
    }
}
#[cfg(feature = "lispify")]
impl Lispify for NamespaceDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(self.path.len() + 1);
        let kind = match self.kind {
            NamespaceKind::Main => "namespace/shared",
            NamespaceKind::Standalone => "namespace/unique",
            NamespaceKind::Test => "namespace/test",
        };
        lisp += Lisp::keyword(kind);
        for id in &self.path {
            lisp += id.lispify();
        }
        lisp
    }
}

impl NamespaceKind {
    /// Get the string representation of the namespace kind
    pub fn as_str(&self) -> &'static str {
        match self {
            NamespaceKind::Main => "namespace",
            NamespaceKind::Standalone => "namespace!",
            NamespaceKind::Test => "namespace?",
            NamespaceKind::Hide => "namespace*",
        }
    }
}
