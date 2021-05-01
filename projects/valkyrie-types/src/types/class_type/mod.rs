use super::*;

use crate::ValkyrieID;
use indexmap::IndexMap;
use nyar_error::FileSpan;
use shredder::Scanner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValkyrieClassType {
    name: ValkyrieID,
    items: IndexMap<String, ValkyrieValue>,
}

impl Hash for ValkyrieClassType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        for (k, v) in self.items.iter() {
            k.hash(state);
            v.hash(state)
        }
    }
}

unsafe impl GcSafe for ValkyrieClassType {}

unsafe impl Scan for ValkyrieClassType {
    fn scan(&self, _: &mut Scanner<'_>) {}
}

impl ValkyrieClassType {
    pub fn new(name: ValkyrieID) -> Self {
        Self { name, items: IndexMap::new() }
    }
    pub fn namespace(&self) -> &[String] {
        self.name.namespace()
    }
    pub fn name(&self) -> &str {
        self.name.name()
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
