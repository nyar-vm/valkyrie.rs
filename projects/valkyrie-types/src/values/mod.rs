use dashu::integer::IBig;
use std::{fmt::Debug, sync::Arc};

mod der;
mod ser;

use crate::{
    builtin::{data_frame::ValkyrieDataFrame, images::ValkyrieImage, ndarray::ValkyrieNDArray},
    JsonValue, ValkyrieClassType, ValkyrieTable, ValkyrieVariantType,
};

#[derive(Clone, Debug)]
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
    Decimal(f64),
    UTF8Character(char),
    UTF8String(Arc<String>),
    Bytes(Arc<Vec<u8>>),
    /// Array, Array2D
    /// ArrayView, ArrayView2D
    Json(Arc<JsonValue>),
    Html(Arc<String>),
    NDArray(Arc<ValkyrieNDArray>),
    Image(Arc<ValkyrieImage>),
    DataFrame(Arc<ValkyrieDataFrame>),
    Table(Arc<ValkyrieTable>),
    Class(Arc<ValkyrieClassType>),
    Variant(Arc<ValkyrieVariantType>),
}
