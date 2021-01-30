use super::*;
use crate::{functions::ValkyriePartialFunction, utils::primitive_type, ValkyrieList};
use im::HashMap;
use indexmap::IndexMap;
use shredder::{marker::GcDrop, Scanner};
use std::{
    collections::{LinkedList, VecDeque},
    fmt::Formatter,
};

#[derive(Clone)]
pub struct ValkyrieDict {
    pub raw: HashMap<ValkyrieValue, ValkyrieValue>,
}

impl Debug for ValkyrieDict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().finish()
    }
}

unsafe impl GcSafe for ValkyrieDict {}
unsafe impl GcDrop for ValkyrieDict {}
unsafe impl Scan for ValkyrieDict {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        for (key, value) in self.raw.iter() {
            scanner.scan(key);
            scanner.scan(value);
        }
    }
}

impl ValkyrieDict {
    pub fn clear(&mut self) {
        self.raw.clear();
    }
}

impl Default for ValkyrieDict {
    fn default() -> Self {
        todo!()
    }
}

impl ValkyrieType for ValkyrieDict {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Dict(self)
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.List");

        Gc::new(this)
    }
}

impl ValkyrieType for () {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Dict(Default::default())
    }

    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Tuple")
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Tuple")
    }
}

impl<T1> ValkyrieType for (T1,)
where
    T1: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::List(ValkyrieList::from_iter(vec![self.0.boxed()]))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        Gc::new(this)
    }
}

impl<T1, T2> ValkyrieType for (T1, T2)
where
    T1: ValkyrieType,
    T2: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::List(ValkyrieList::from_iter(vec![self.0.boxed(), self.1.boxed()]))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        Gc::new(this)
    }
}

impl<T1, T2, T3> ValkyrieType for (T1, T2, T3)
where
    T1: ValkyrieType,
    T2: ValkyrieType,
    T3: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::List(ValkyrieList::from_iter(vec![self.0.boxed(), self.1.boxed(), self.2.boxed()]))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        this.mut_generic_types().push(T3::static_type());
        Gc::new(this)
    }
}

impl<T1, T2, T3, T4> ValkyrieType for (T1, T2, T3, T4)
where
    T1: ValkyrieType,
    T2: ValkyrieType,
    T3: ValkyrieType,
    T4: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::List(ValkyrieList::from_iter(vec![self.0.boxed(), self.1.boxed(), self.2.boxed(), self.3.boxed()]))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        this.mut_generic_types().push(T3::static_type());
        this.mut_generic_types().push(T4::static_type());
        Gc::new(this)
    }
}

impl<T1, T2, T3, T4, T5> ValkyrieType for (T1, T2, T3, T4, T5)
where
    T1: ValkyrieType,
    T2: ValkyrieType,
    T3: ValkyrieType,
    T4: ValkyrieType,
    T5: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::List(ValkyrieList::from_iter(vec![
            self.0.boxed(),
            self.1.boxed(),
            self.2.boxed(),
            self.3.boxed(),
            self.4.boxed(),
        ]))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        this.mut_generic_types().push(T3::static_type());
        this.mut_generic_types().push(T4::static_type());
        this.mut_generic_types().push(T5::static_type());
        Gc::new(this)
    }
}
