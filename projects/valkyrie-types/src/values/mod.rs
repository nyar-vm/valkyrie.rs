use shredder::{marker::GcSafe, Gc, Scan, Scanner};
use std::{fmt::Debug, sync::Arc};
use valkyrie_ast::helper::ValkyrieNode;

mod der;
mod ser;

#[cfg(feature = "polars")]
use crate::builtin::data_frame::ValkyrieDataFrame;
use crate::{
    builtin::{images::ValkyrieImage, ndarray::ValkyrieNDArray},
    ValkyrieClassType, ValkyrieDict, ValkyrieList, ValkyrieNumber, ValkyrieVariantType,
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
    Number(ValkyrieNumber),
    Unicode(char),
    UTF8String(Gc<String>),
    Bytes(Gc<Vec<u8>>),
    Html(Gc<String>),
    NDArray(Gc<ValkyrieNDArray>),
    Image(Gc<ValkyrieImage>),
    #[cfg(feature = "polars")]
    DataFrame(Gc<ValkyrieDataFrame>),
    List(ValkyrieList),
    Dict(ValkyrieDict),
    Class(Gc<ValkyrieClassType>),
    Variant(Gc<ValkyrieVariantType>),
}

unsafe impl GcSafe for ValkyrieValue {}

unsafe impl Scan for ValkyrieValue {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        match self {
            ValkyrieValue::Nothing => {}
            ValkyrieValue::Uninitialized => {}
            ValkyrieValue::Null => {}
            ValkyrieValue::Unit => {}
            ValkyrieValue::Boolean(_) => {}
            ValkyrieValue::Number(v) => scanner.scan(v),
            ValkyrieValue::Unicode(_) => {}
            ValkyrieValue::UTF8String(v) => scanner.scan(v),
            ValkyrieValue::Bytes(v) => scanner.scan(v),
            ValkyrieValue::Html(_) => {}
            ValkyrieValue::NDArray(_) => {}
            ValkyrieValue::Image(_) => {}
            #[cfg(feature = "polars")]
            ValkyrieValue::DataFrame(_) => {}
            ValkyrieValue::List(v) => scanner.scan(v),
            ValkyrieValue::Dict(v) => scanner.scan(v),
            ValkyrieValue::Class(_) => {}
            ValkyrieValue::Variant(_) => {}
        }
    }
}
