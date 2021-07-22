use super::*;

pub struct ValkyrieSymbol {
    pub(crate) path: Vec<Arc<str>>,
    pub(crate) span: FileSpan,
}

impl Debug for ValkyrieSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path.join("∷"))
    }
}

trait AsValkyrieSymbol {
    fn as_valkyrie_symbol(&self) -> ValkyrieSymbol;
}

impl AsValkyrieSymbol for NamePathNode {
    fn as_valkyrie_symbol(&self) -> ValkyrieSymbol {
        ValkyrieSymbol { path: self.path.iter().map(|s| Arc::from(s.name.as_str())).collect(), span: self.span.clone() }
    }
}

impl ValkyrieSymbol {
    pub fn new<S: AsValkyrieSymbol>(id: S) -> Self {
        id.as_valkyrie_symbol()
    }
    pub fn with_span(self, span: FileSpan) -> Self {
        Self { span, ..self }
    }
}
