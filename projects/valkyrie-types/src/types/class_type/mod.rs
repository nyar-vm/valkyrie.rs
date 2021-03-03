use super::*;

use indexmap::IndexMap;
use nyar_error::FileSpan;
use shredder::Scanner;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ValkyrieSymbol {
    namepath: Vec<String>,
    location: FileSpan,
}

impl ValkyrieSymbol {
    pub fn namepath(&self) -> &[String] {
        &self.namepath
    }
    pub fn namespace(&self) -> &[String] {
        assert!(self.namepath.len() > 0);
        unsafe { self.namepath.get_unchecked(..self.namepath.len() - 1) }
    }
    pub fn name(&self) -> &str {
        self.namepath.last().unwrap_or_default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValkyrieClassType {
    name: ValkyrieSymbol,
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
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: ValkyrieSymbol { namepath: Vec::new(), location: name.into() }, items: IndexMap::new() }
    }
    pub fn set_namespace<I>(&mut self, space: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.name.namepath = space.into_iter().collect();
    }
    pub fn with_namespace<I>(mut self, space: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        self.set_namespace(space);
        self
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
