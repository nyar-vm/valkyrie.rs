use shredder::{marker::GcSafe, Gc, Scan, Scanner};
use std::{fmt::Debug, sync::Arc};
use valkyrie_ast::helper::ValkyrieNode;

mod der;
mod ser;

#[cfg(feature = "polars")]
use crate::builtin::data_frame::ValkyrieDataFrame;
use crate::{
    builtin::{images::ValkyrieImage, ndarray::ValkyrieNDArray},
    JsonValue, ValkyrieClassType, ValkyrieNumber, ValkyrieTable, ValkyrieVariantType,
};

#[derive(Clone, Debug)]
pub enum ValkyrieValue {
    /// ADT = -1
    Nothing,
    /// An uninitialized value, null for pointer types, and default for value types
    ///
    /// Trying to read from an uninitialized value is a fatal error.
    Uninitialized,
    /// ADT = 0
    Null,
    /// ADT = 1
    Unit,
    /// ADT = 2
    ///
    /// Native boolean type, 8bit
    Boolean(bool),
    Number(Gc<ValkyrieNumber>),
    Unicode(char),
    UTF8String(Gc<String>),
    Bytes(Gc<Vec<u8>>),
    Html(Gc<String>),
    NDArray(Gc<ValkyrieNDArray>),
    Image(Gc<ValkyrieImage>),
    #[cfg(feature = "polars")]
    DataFrame(Gc<ValkyrieDataFrame>),
    Table(Gc<ValkyrieTable>),
    Class(Gc<ValkyrieClassType>),
    Variant(Gc<ValkyrieVariantType>),
}
