use super::*;
use itertools::Itertools;

impl Debug for ResolveContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolveContext")
            .field("namespace", &self.namespace.join("::"))
            .field("document", &self.document)
            .field("unsolved", &self.unsolved)
            .field("items", &self.items)
            .field("errors", &self.errors)
            .field("main_function", &self.main_function)
            .finish()
    }
}
impl Debug for ModuleItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resource(v) => Debug::fmt(v, f),
            Self::External(v) => Debug::fmt(v, f),
            // Self::Imported(v) => Debug::fmt(v, f),
            Self::Structure(v) => Debug::fmt(v, f),
            Self::Variant(v) => Debug::fmt(v, f),
            // Self::Function(v) => Debug::fmt(v, f),
        }
    }
}
