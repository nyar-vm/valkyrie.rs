use dashu::integer::IBig;
use std::sync::Arc;

mod der;
mod ser;

use crate::{
    builtin::{data_frame::ValkyrieDataFrame, images::ValkyrieImage, ndarray::ValkyrieNDArray},
    JsonValue, ValkyrieClassType, ValkyrieDataTable, ValkyrieVariantType,
};

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
    Bytes(Arc<Vec<u8>>),
    /// Array, Array2D
    /// ArrayView, ArrayView2D
    Json(Arc<JsonValue>),
    NDArray(Arc<ValkyrieNDArray>),
    Image(Arc<ValkyrieImage>),
    DataFrame(Arc<ValkyrieDataFrame>),
    DataTable(Arc<ValkyrieDataTable>),
    Class(Arc<ValkyrieClassType>),
    Variant(Arc<ValkyrieVariantType>),
}
