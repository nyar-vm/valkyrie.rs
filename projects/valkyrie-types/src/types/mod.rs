use crate::{
    types::atomic_type::ValkyrieDocument,
    utils::{primitive_type, Namepath},
    ValkyrieClassType, ValkyrieDict, ValkyrieValue,
};
use indexmap::IndexMap;
use itertools::Itertools;
use nyar_collection::NyarTuple;
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    any::type_name,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
};

pub mod atomic_type;
pub mod class_type;
pub mod literal_type;
pub mod tuple_type;
pub mod union_type;
pub mod variant_type;

// rtti of valkyrie type
#[derive(Clone, Debug, Default, Eq, PartialEq, Scan)]
pub struct ValkyrieMetaType {
    namepath: Namepath,
    document: ValkyrieDocument,
    generic_types: Vec<Gc<ValkyrieMetaType>>,
}

impl Default for ValkyrieValue {
    fn default() -> Self {
        ValkyrieValue::Null
    }
}

impl ValkyrieType for ValkyrieValue {
    fn boxed(self) -> Self {
        self
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        match self {
            Self::Uninitialized => primitive_type("std.primitive.Uninitialized"),
            Self::Nothing => primitive_type("std.primitive.Never"),
            Self::Null => primitive_type("std.primitive.Null"),
            Self::Unit => primitive_type("std.primitive.Unit"),
            // Self::Boolean(v) => v.dynamic_type(),
            // Self::Number(v) => v.dynamic_type(),
            // Self::Unicode(v) => v.dynamic_type(),
            // Self::UTF8String(v) => v.dynamic_type(),
            // Self::Bytes(v) => v.dynamic_type(),
            // Self::Class(v) => v.dynamic_type(),
            // Self::Variant(v) => v.dynamic_type(),
            // Self::NDArray(v) => v.dynamic_type(),
            // Self::Image(v) => v.dynamic_type(),
            // #[cfg(feature = "polars")]
            // Self::DataFrame(v) => v.dynamic_type(),
            // Self::Table(v) => v.dynamic_type(),
            // Self::Html(_) => primitive_type("html.Html"),
            _ => todo!(),
        }
    }
}

impl ValkyrieMetaType {
    pub fn new(namepath: &str) -> Self {
        let mut o = Self::default();
        o.set_namepath(namepath);
        o
    }

    pub fn set_namepath(&mut self, namepath: &str) {
        for s in namepath.split('.') {
            self.namepath.push(s);
        }
    }
    pub fn mut_namepath(&mut self) -> &mut Namepath {
        &mut self.namepath
    }
    pub fn mut_generic_types(&mut self) -> &mut Vec<Gc<ValkyrieMetaType>> {
        &mut self.generic_types
    }
}

#[allow(clippy::wrong_self_convention)]
pub trait ValkyrieType
where
    Self: Sized,
{
    fn boxed(self) -> ValkyrieValue;

    fn static_type() -> Gc<ValkyrieMetaType> {
        panic!("{} does not a static type", type_name::<Self>())
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType>;
}

impl ValkyrieMetaType {
    pub fn name(&self) -> &str {
        assert_ne!(self.namepath.length(), 0, "namepath `{:?}` is not valid", self.namepath);
        self.namepath.name()
    }
    pub fn namespace(&self, join: &str) -> String {
        assert_ne!(self.namepath.length(), 0, "namepath `{:?}` is not valid", self.namepath);
        self.namepath.namespace().join(join)
    }
    pub fn display_type(&self, full_path: bool) -> String {
        let this = match full_path {
            true => self.namepath.join("::"),
            false => self.name().to_string(),
        };
        if self.generic_types.is_empty() {
            return this;
        }
        format!(
            "{}[{}]",
            this,
            self.generic_types.iter().map(|f| f.get().dynamic_type().get().display_type(full_path)).join(", ")
        )
    }
}

impl Display for ValkyrieMetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
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

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        Gc::new(self.clone())
    }
}
