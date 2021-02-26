use crate::values::ValkyrieValue;
use std::mem::MaybeUninit;
use wasmtime::{
    component::{
        ComponentType, Lift, Lower,
        __internal::{anyhow::bail, CanonicalAbiInfo, InstanceType, InterfaceType, LiftContext, LowerContext},
    },
    ValRaw,
};

pub enum ValkyrieNumber {
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Float32(f32),
    Float64(f64),
}

impl From<u8> for ValkyrieNumber {
    #[inline]
    fn from(value: u8) -> Self {
        Self::Unsigned8(value)
    }
}

impl From<u8> for ValkyrieValue {
    #[inline]
    fn from(value: u8) -> Self {
        Self::Number(ValkyrieNumber::from(value))
    }
}

macro_rules! map_value {

}


map_value! {
    u8 => Unsigned8,
    u16 => Unsigned16,

}
