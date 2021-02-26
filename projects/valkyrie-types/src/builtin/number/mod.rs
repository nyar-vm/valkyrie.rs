use std::mem::MaybeUninit;
use wasmtime::{
    component::{
        ComponentType, Lift, Lower,
        __internal::{CanonicalAbiInfo, InstanceType, InterfaceType, LiftContext, LowerContext},
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

unsafe impl ComponentType for ValkyrieNumber {
    type Lower = ValRaw;
    const ABI: CanonicalAbiInfo = Default::default();

    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> wasmtime::Result<()> {
        todo!()
    }
}

unsafe impl Lift for ValkyrieNumber {
    fn lift(cx: &mut LiftContext<'_>, ty: InterfaceType, src: &Self::Lower) -> wasmtime::Result<Self> {
        match ty {
            InterfaceType::Bool => {}
            InterfaceType::S8 => {}
            InterfaceType::U8 => {}
            InterfaceType::S16 => {}
            InterfaceType::U16 => {}
            InterfaceType::S32 => {}
            InterfaceType::U32 => {}
            InterfaceType::S64 => {}
            InterfaceType::U64 => {}
            InterfaceType::Float32 => {}
            InterfaceType::Float64 => {}
            InterfaceType::Char => {}
            InterfaceType::String => {}
            InterfaceType::Record(_) => {}
            InterfaceType::Variant(_) => {}
            InterfaceType::List(_) => {}
            InterfaceType::Tuple(_) => {}
            InterfaceType::Flags(_) => {}
            InterfaceType::Enum(_) => {}
            InterfaceType::Option(_) => {}
            InterfaceType::Result(_) => {}
            InterfaceType::Own(_) => {}
            InterfaceType::Borrow(_) => {}
        }
    }

    fn load(cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> wasmtime::Result<Self> {
        todo!()
    }
}

unsafe impl Lower for ValkyrieNumber {
    fn lower<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> wasmtime::Result<()> {
        match self {
            Self::Unsigned8(v) => v.lower(cx, ty, dst),
            Self::Unsigned16(v) => v.lower(cx, ty, dst),
            Self::Unsigned32(v) => v.lower(cx, ty, dst),
            Self::Unsigned64(v) => v.lower(cx, ty, dst),
            Self::Integer8(v) => v.lower(cx, ty, dst),
            Self::Integer16(v) => v.lower(cx, ty, dst),
            Self::Integer32(v) => v.lower(cx, ty, dst),
            Self::Integer64(v) => v.lower(cx, ty, dst),
            Self::Float32(v) => v.lower(cx, ty, dst),
            Self::Float64(v) => v.lower(cx, ty, dst),
        }
    }

    fn store<T>(&self, cx: &mut LowerContext<'_, T>, ty: InterfaceType, offset: usize) -> wasmtime::Result<()> {
        match self {
            Self::Unsigned8(v) => v.store(cx, ty, offset),
            Self::Unsigned16(v) => v.store(cx, ty, offset),
            Self::Unsigned32(v) => v.store(cx, ty, offset),
            Self::Unsigned64(v) => v.store(cx, ty, offset),
            Self::Integer8(v) => v.store(cx, ty, offset),
            Self::Integer16(v) => v.store(cx, ty, offset),
            Self::Integer32(v) => v.store(cx, ty, offset),
            Self::Integer64(v) => v.store(cx, ty, offset),
            Self::Float32(v) => v.store(cx, ty, offset),
            Self::Float64(v) => v.store(cx, ty, offset),
        }
    }
}
