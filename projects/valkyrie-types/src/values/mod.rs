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
    Result(Result<ValkyrieValue, ValkyrieValue>)
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
                Maybe::
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

unsafe impl ComponentNamedList for ValkyrieValue {}

unsafe impl ComponentType for ValkyrieValue {
    type Lower = ValRaw;
    const ABI: CanonicalAbiInfo = Default::default();

    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> wasmtime::Result<()> {
        todo!()
    }
}

unsafe impl Lift for ValkyrieValue {
    fn lift(cx: &mut LiftContext<'_>, ty: InterfaceType, src: &Self::Lower) -> wasmtime::Result<Self> {
        match ty {
            InterfaceType::Bool => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::S8 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::U8 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::S16 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::U16 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::S32 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::U32 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::S64 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::U64 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::Float32 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::Float64 => bool::lift(cx, ty, src).map(Into::into),
            InterfaceType::Char => char::lift(),
            InterfaceType::String => String::lift(cx, ty, src),
            InterfaceType::Record(_) => {
                todo!()
            }
            InterfaceType::Variant(_) => {
                todo!()
            }
            InterfaceType::List(_) => {
                todo!()
            }
            InterfaceType::Tuple(_) => {
                todo!()
            }
            InterfaceType::Flags(_) => {
                todo!()
            }
            InterfaceType::Enum(_) => {
                todo!()
            }
            InterfaceType::Option(_) => {
                todo!()
            }
            InterfaceType::Result(_) => {
                todo!()
            }
            InterfaceType::Own(_) => {
                todo!()
            }
            InterfaceType::Borrow(_) => {
                todo!()
            }
        }
    }

    fn load(cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> wasmtime::Result<Self> {
        match ty {
            InterfaceType::Bool => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::S8 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::U8 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::S16 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::U16 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::S32 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::U32 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::S64 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::U64 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::Float32 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::Float64 => bool::load(cx, ty, bytes).map(Into::into),
            InterfaceType::Char => {
                todo!()
            }
            InterfaceType::String => {
                todo!()
            }
            InterfaceType::Record(_) => {
                todo!()
            }
            InterfaceType::Variant(_) => {
                todo!()
            }
            InterfaceType::List(_) => {
                todo!()
            }
            InterfaceType::Tuple(_) => {
                todo!()
            }
            InterfaceType::Flags(_) => {
                todo!()
            }
            InterfaceType::Enum(_) => {
                todo!()
            }
            InterfaceType::Option(_) => {
                todo!()
            }
            InterfaceType::Result(_) => {
                todo!()
            }
            InterfaceType::Own(_) => {
                todo!()
            }
            InterfaceType::Borrow(_) => {
                todo!()
            }
        }
    }
}

unsafe impl Lower for ValkyrieValue {
    fn lower<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> wasmtime::Result<()> {
        match self {
            ValkyrieValue::Nothing => {
                todo!()
            }
            ValkyrieValue::Unit => vec![].lower(cx, ty, dst),
            ValkyrieValue::Boolean(v) => v.lower(cx, ty, dst),
            ValkyrieValue::Number(v) => match v {
                ValkyrieNumber::Unsigned8(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Unsigned16(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Unsigned32(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Unsigned64(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Integer8(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Integer16(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Integer32(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Integer64(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Float32(v) => v.lower(cx, ty, dst),
                ValkyrieNumber::Float64(v) => v.lower(cx, ty, dst),
            },
        }
    }

    fn store<T>(&self, cx: &mut LowerContext<'_, T>, ty: InterfaceType, offset: usize) -> wasmtime::Result<()> {
        match self {
            ValkyrieValue::Nothing => {
                todo!()
            }
            ValkyrieValue::Unit => ().store(cx, ty, offset),
            ValkyrieValue::Boolean(v) => v.store(cx, ty, offset),
            ValkyrieValue::Number(v) => match v {
                ValkyrieNumber::Unsigned8(v) => v.v(cx, ty, offset),
                ValkyrieNumber::Unsigned16(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Unsigned32(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Unsigned64(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Integer8(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Integer16(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Integer32(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Integer64(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Float32(v) => v.store(cx, ty, offset),
                ValkyrieNumber::Float64(v) => v.store(cx, ty, offset),
            },
        }
    }
}
