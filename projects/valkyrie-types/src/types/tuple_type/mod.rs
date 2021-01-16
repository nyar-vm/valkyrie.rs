use super::*;
use crate::{functions::ValkyriePartialFunction, utils::primitive_type};
use indexmap::IndexMap;
use std::collections::{LinkedList, VecDeque};

#[derive(Clone, Debug, Scan)]
pub struct ValkyrieTable {
    tuple: bool,
    items: VecDeque<ValkyrieValue>,
    pairs: IndexMap<ValkyrieValue, ValkyrieValue>,
}

#[derive(Clone, Debug, Default)]
pub struct ValkyrieList {
    tuple: bool,
    /// Type bound if this is a homogeneous list
    array: Option<Gc<ValkyrieMetaType>>,
    /// items, homogeneous or heterogeneous
    items: LinkedList<ValkyrieValue>,
}

pub struct ValkyrieDict {
    pairs: IndexMap<String, ValkyrieValue>,
}

impl ValkyrieList {
    /// `[a] ++ new`
    /// ```vk
    /// append<T>(mut self, ..items: List<T>, <) { }
    /// ```
    pub fn append(&mut self, item: ValkyrieValue) {
        self.items.push_back(item);
    }
    /// `[a] ++ [new]`
    pub fn append_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = ValkyrieValue>,
    {
        self.items.extend(items);
    }
    /// `new ++ [a]`
    pub fn prepend(&mut self, item: ValkyrieValue) {
        self.items.push_front(item);
    }
    /// `[new] ++ [a]`
    pub fn prepend_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = ValkyrieValue>,
    {
        self.items.extend(items);
    }
}

impl ValkyrieTable {
    pub fn list() -> Self {
        Self { tuple: false, items: VecDeque::new(), pairs: Default::default() }
    }

    pub fn tuple() -> Self {
        Self { tuple: true, items: VecDeque::new(), pairs: Default::default() }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.pairs.clear();
    }

    pub fn extend_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = ValkyrieValue>,
    {
        self.items.extend(items);
    }

    pub fn extend_one(&mut self, item: ValkyrieValue) {
        self.items.push_back(item);
    }
}

impl Default for ValkyrieTable {
    fn default() -> Self {
        todo!()
    }
}

impl ValkyrieType for ValkyrieTable {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Table(Arc::new(self))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.List");

        Arc::new(this)
    }
}

impl ValkyrieType for () {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Table(Arc::new(ValkyrieTable::tuple()))
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
        ValkyrieValue::Table(Arc::new(ValkyrieTable::tuple()))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        Arc::new(this)
    }
}

impl<T1, T2> ValkyrieType for (T1, T2)
where
    T1: ValkyrieType,
    T2: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Table(Arc::new(ValkyrieTable::tuple()))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        Arc::new(this)
    }
}

impl<T1, T2, T3> ValkyrieType for (T1, T2, T3)
where
    T1: ValkyrieType,
    T2: ValkyrieType,
    T3: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Table(Arc::new(ValkyrieTable::tuple()))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        this.mut_generic_types().push(T3::static_type());
        Arc::new(this)
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
        ValkyrieValue::Table(Arc::new(ValkyrieTable::tuple()))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        this.mut_generic_types().push(T3::static_type());
        this.mut_generic_types().push(T4::static_type());
        Arc::new(this)
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
        ValkyrieValue::Table(Arc::new(ValkyrieTable::tuple()))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Tuple");
        this.mut_generic_types().push(T1::static_type());
        this.mut_generic_types().push(T2::static_type());
        this.mut_generic_types().push(T3::static_type());
        this.mut_generic_types().push(T4::static_type());
        this.mut_generic_types().push(T5::static_type());
        Arc::new(this)
    }
}
