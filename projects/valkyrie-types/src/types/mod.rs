use std::{
    any::type_name,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    sync::Arc,
};

use itertools::Itertools;

use crate::{
    types::atomic_type::ValkyrieDocument, utils::primitive_type, ValkyrieAtomicType, ValkyrieClass, ValkyrieClassType,
    ValkyrieValue, ValkyrieVariantType,
};

pub mod atomic_type;
pub mod class_type;
pub mod literal_type;
pub mod tuple_type;
pub mod union_type;
pub mod variant_type;

// rtti of valkyrie type
#[derive(Clone, Debug, Default)]
pub struct ValkyrieMetaType {
    namepath: Vec<String>,
    document: ValkyrieDocument,
    generic_types: Vec<Arc<ValkyrieMetaType>>,
}

impl Default for ValkyrieValue {
    fn default() -> Self {
        ValkyrieValue::Never
    }
}

impl ValkyrieType for ValkyrieValue {
    fn boxed(self) -> ValkyrieValue {
        self
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        match self {
            ValkyrieValue::Never => primitive_type("std.primitive.Never"),
            ValkyrieValue::Null => primitive_type("std.primitive.Null"),
            ValkyrieValue::Boolean(v) => v.dynamic_type(),
            ValkyrieValue::Unsigned8(v) => v.dynamic_type(),
            ValkyrieValue::Unsigned16(v) => v.dynamic_type(),
            ValkyrieValue::Unsigned32(v) => v.dynamic_type(),
            ValkyrieValue::Unsigned64(v) => v.dynamic_type(),
            ValkyrieValue::Unsigned128(v) => v.dynamic_type(),
            ValkyrieValue::Integer8(v) => v.dynamic_type(),
            ValkyrieValue::Integer16(v) => v.dynamic_type(),
            ValkyrieValue::Integer32(v) => v.dynamic_type(),
            ValkyrieValue::Integer64(v) => v.dynamic_type(),
            ValkyrieValue::Integer128(v) => v.dynamic_type(),
            ValkyrieValue::Float32(v) => v.dynamic_type(),
            ValkyrieValue::Float64(v) => v.dynamic_type(),
            ValkyrieValue::Buffer(v) => v.dynamic_type(),
            ValkyrieValue::String(v) => v.dynamic_type(),
            ValkyrieValue::Class(v) => v.dynamic_type(),
            ValkyrieValue::Variant(v) => v.dynamic_type(),
        }
    }
}

impl ValkyrieMetaType {
    pub fn set_namepath(&mut self, namepath: &str) {
        self.namepath.clear();
        for s in namepath.split('.') {
            self.namepath.push(s.to_string());
        }
    }
    pub fn mut_namepath(&mut self) -> &mut Vec<String> {
        &mut self.namepath
    }
    pub fn mut_generic_types(&mut self) -> &mut Vec<Arc<ValkyrieMetaType>> {
        &mut self.generic_types
    }
}

#[allow(clippy::wrong_self_convention)]
pub trait ValkyrieType
where
    Self: Sized,
{
    fn boxed(self) -> ValkyrieValue;

    fn static_type() -> Arc<ValkyrieMetaType> {
        panic!("{} does not a static type", type_name::<Self>())
    }
    fn dynamic_type(&self) -> Arc<ValkyrieMetaType>;
}

impl ValkyrieMetaType {
    pub fn name(&self) -> String {
        assert_ne!(self.namepath.len(), 0, "namepath `{:?}` is not valid", self.namepath);
        self.namepath.last().unwrap().to_owned()
    }
    pub fn namespace(&self, join: &str) -> String {
        assert_ne!(self.namepath.len(), 0, "namepath `{:?}` is not valid", self.namepath);
        self.namepath[..self.namepath.len() - 1].join(join)
    }
    pub fn display_type(&self, full_path: bool) -> String {
        let this = match full_path {
            true => self.namepath.join("::"),
            false => self.name(),
        };
        if self.generic_types.is_empty() {
            return this;
        }
        format!("{}[{}]", this, self.generic_types.iter().map(|f| f.dynamic_type().display_type(full_path)).join(", "))
    }
}

impl Display for ValkyrieMetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display_type(false))
    }
}

impl Hash for ValkyrieMetaType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.display_type(true).as_bytes());
    }
}

impl ValkyrieType for ValkyrieMetaType {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        Arc::new(self.clone())
    }
}
