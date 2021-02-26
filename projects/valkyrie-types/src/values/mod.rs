// mod der;
// mod ser;

use crate::ValkyrieNumber;
use std::mem::MaybeUninit;
use wasmtime::{
    component::{
        ComponentNamedList, ComponentType, Lift, Lower,
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
        todo!()
    }

    fn load(cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> wasmtime::Result<Self> {
        todo!()
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
            ValkyrieValue::Nothing => {}
            ValkyrieValue::Unit => {}
            ValkyrieValue::Boolean(v) => v.lower(cx, ty, dst),
            ValkyrieValue::Number(v) => {}
        }
    }

    fn store<T>(&self, cx: &mut LowerContext<'_, T>, ty: InterfaceType, offset: usize) -> wasmtime::Result<()> {
        todo!()
    }
}
