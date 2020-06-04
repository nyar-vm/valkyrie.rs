use super::*;

pub struct ValkyrieClass {
    tuple: bool,
    items: Vec<ValkyrieValue>,
}

impl ValkyrieClass {
    pub fn list() -> Self {
        Self { tuple: false, items: Vec::new() }
    }

    pub fn tuple() -> Self {
        Self { tuple: true, items: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn extend_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = ValkyrieValue>,
    {
        self.items.extend(items);
    }

    pub fn extend_one(&mut self, item: ValkyrieValue) {
        self.items.push(item);
    }
}

impl Default for ValkyrieClass {
    fn default() -> Self {
        todo!()
    }
}

impl ValkyrieType for ValkyrieClass {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Class(Arc::new(self))
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.List");

        Arc::new(this)
    }
}
