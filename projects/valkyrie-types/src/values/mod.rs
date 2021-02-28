// mod der;
// mod ser;

use crate::ValkyrieNumber;
use std::mem::MaybeUninit;
use wasmtime::{
    component::{
        ComponentNamedList, ComponentType, Lift, Lower, Val,
        __internal::{CanonicalAbiInfo, InstanceType, InterfaceType, LiftContext, LowerContext},
    },
    ValRaw,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ValkyrieValue {
    /// ADT = -1
    Nothing,
    /// An uninitialized value, null for pointer types, and default for value types
    ///
    /// Trying to read from an uninitialized value is a fatal error.
    Unit,
    /// ADT = 2
    ///
    /// Native boolean type, 8bit
    Boolean(bool),
    Number(ValkyrieNumber),
    Result(Result<ValkyrieValue, ValkyrieValue>),
}

impl From<Val> for ValkyrieValue {
    fn from(value: Val) -> Self {
        match value {
            Val::Bool(v) => v.into(),
            Val::S8(v) => v.into(),
            Val::U8(v) => v.into(),
            Val::S16(v) => v.into(),
            Val::U16(v) => v.into(),
            Val::S32(v) => v.into(),
            Val::U32(v) => v.into(),
            Val::S64(v) => v.into(),
            Val::U64(v) => v.into(),
            Val::Float32(v) => v.into(),
            Val::Float64(v) => v.into(),
            Val::Char(_) => {
                todo!()
            }
            Val::String(_) => {
                todo!()
            }
            Val::List(_) => {
                todo!()
            }
            Val::Record(_) => {
                todo!()
            }
            Val::Tuple(_) => {
                todo!()
            }
            Val::Variant(_) => {
                todo!()
            }
            Val::Enum(_) => {
                todo!()
            }
            Val::Option(v) => {
                todo!()
            }
            Val::Result(_) => {
                todo!()
            }
            Val::Flags(_) => {
                todo!()
            }
            Val::Resource(_) => {
                todo!()
            }
        }
    }
}
