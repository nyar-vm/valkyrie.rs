use std::sync::Arc;
mod der;
mod ser;

use crate::{ValkyrieClass, ValkyrieVariantType};

pub enum ValkyrieValue {
    /// ADT = -1
    Nothing,
    /// ADT = 0
    Null,
    /// ADT = 1
    Unit,
    /// ADT = 2
    ///
    /// Native boolean type, 8bit
    Boolean(bool),
    /// Native number type, 8bit
    Unsigned8(u8),
    /// Native number type, 16bit
    Unsigned16(u16),
    /// Native number type, 32bit
    Unsigned32(u32),
    /// Native number type, 64bit
    Unsigned64(u64),
    /// Native number type, 128bit
    Unsigned128(u128),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    Float32(f32),
    Float64(f64),
    String(Arc<String>),
    Buffer(Arc<Vec<u8>>),
    Class(Arc<ValkyrieClass>),
    Variant(Arc<ValkyrieVariantType>),
}
