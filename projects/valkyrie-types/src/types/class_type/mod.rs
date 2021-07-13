use super::*;
use crate::ValkyrieString;
use indexmap::IndexMap;
use nyar_error::FileSpan;
use std::sync::Arc;
use valkyrie_ast::{IdentifierNode, NamePathNode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValkyrieClassType {
    symbol: Vec<Arc<str>>,
    items: IndexMap<String, ValkyrieValue>,
    span: FileSpan,
}

impl Hash for ValkyrieClassType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        for (k, v) in self.items.iter() {
            k.hash(state);
            v.hash(state)
        }
    }
}

impl ValkyrieClassType {
    pub fn new(space: &NamePathNode, name: &IdentifierNode) -> Self {
        let mut symbol = Vec::with_capacity(space.path.len() + 1);
        symbol.extend(space.path.iter().map(|s| Arc::from(s.name.as_str())));
        symbol.extend_one(Arc::from(name.name.as_str()));
        Self { symbol, items: Default::default(), span: Default::default() }
    }
    pub fn name(&self) -> String {
        self.symbol.join("∷")
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn extend_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = ValkyrieValue>,
    {
        todo!()
    }

    pub fn extend_one(&mut self, item: ValkyrieValue) {
        todo!()
    }
}

impl Default for ValkyrieClassType {
    fn default() -> Self {
        todo!()
    }
}

impl ValkyrieType for ValkyrieClassType {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.List");

        Gc::new(this)
    }
}
