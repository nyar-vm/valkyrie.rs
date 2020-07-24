use dashu::integer::IBig;
use polars::frame::DataFrame;
use std::sync::Arc;

mod der;
mod ser;

use crate::{
    builtin::{data_frame::ValkyrieDataFrame, figures::ValkyrieImage, ndarray::ValkyrieNDArray},
    ValkyrieClassType, ValkyrieDataTable, ValkyrieVariantType,
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
    Json(Arc<serde_json::Value>),
    NDArray(Arc<ValkyrieNDArray>),
    Image(Arc<ValkyrieImage>),
    DataFrame(Arc<ValkyrieDataFrame>),
    DataTable(Arc<ValkyrieDataTable>),
    Class(Arc<ValkyrieClassType>),
    Variant(Arc<ValkyrieVariantType>),
}
