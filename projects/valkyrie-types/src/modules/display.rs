use super::*;
use itertools::Itertools;
use std::collections::BTreeMap;

impl Debug for ResolveContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let imports: BTreeMap<_, _> = self.using.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        f.debug_struct("ResolveContext")
            .field("namespace", &self.namespace.join("∷"))
            .field("document", &self.document)
            .field("imports", &imports)
            .field("items", &self.items.values())
            .field("errors", &self.errors)
            .field("main_function", &self.main_function)
            .finish()
    }
}
impl Debug for ModuleItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(v) => Debug::fmt(v, f),
            // Self::Imported(v) => Debug::fmt(v, f),
            Self::Structure(v) => Debug::fmt(v, f),
            Self::Variant(v) => Debug::fmt(v, f),
            // Self::Function(v) => Debug::fmt(v, f),
        }
    }
}
