use dashu::{float::FBig, integer::IBig, rational::RBig};
use std::{fmt::Debug, sync::Arc};

mod der;
mod ser;

#[cfg(feature = "polars")]
use crate::builtin::data_frame::ValkyrieDataFrame;
use crate::{
    builtin::{images::ValkyrieImage, ndarray::ValkyrieNDArray},
    JsonValue, ValkyrieClassType, ValkyrieTable, ValkyrieVariantType,
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
    Integer(IBig),
    Decimal(FBig),
    Unicode(char),
    UTF8String(Arc<String>),
    Bytes(Arc<Vec<u8>>),
    /// Array, Array2D
    /// ArrayView, ArrayView2D
    Json(Arc<JsonValue>),
    Html(Arc<String>),
    NDArray(Arc<ValkyrieNDArray>),
    Image(Arc<ValkyrieImage>),
    #[cfg(feature = "polars")]
    DataFrame(Arc<ValkyrieDataFrame>),
    Table(Arc<ValkyrieTable>),
    Class(Arc<ValkyrieClassType>),
    Variant(Arc<ValkyrieVariantType>),
}
