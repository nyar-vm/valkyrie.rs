use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for NamespaceDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let head = theme.keyword(self.kind.as_str());
        let space = " ";
        let path = theme.join(self.path.clone(), ".");
        let semi = ";";
        head.append(space).append(path).append(semi)
    }
}

impl NamespaceKind {
    /// Get the string representation of the namespace kind
    pub fn as_str(&self) -> &'static str {
        match self {
            NamespaceKind::Shared => "namespace",
            NamespaceKind::Unique => "namespace!",
            NamespaceKind::Test => "namespace?",
        }
    }
}
