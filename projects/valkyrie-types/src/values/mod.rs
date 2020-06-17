use dashu::integer::IBig;
use std::sync::Arc;

mod der;
mod ser;

use crate::{ValkyrieTable, ValkyrieVariantType};

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
    Integer(IBig),
    Float32(f32),
    Float64(f64),
    UTF8Character(char),
    UTF8String(Arc<String>),
    Buffer(Arc<Vec<u8>>),
    Class(Arc<ValkyrieTable>),
    Variant(Arc<ValkyrieVariantType>),
}
