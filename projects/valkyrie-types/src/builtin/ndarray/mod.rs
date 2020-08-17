use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};
use ndarray::{Array, Array2};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ValkyrieNDArray {
    inner_bytes: Array2<u8>,
    data_type: NDArrayDataType,
    dimension: Vec<usize>,
}

#[derive(Clone, Debug)]
pub enum NDArrayDataType {
    Float32,
    Float64,
}

impl ValkyrieNDArray {
    pub fn recast<A, D>(&self) -> Array<A, D> {
        todo!()
    }
}

impl ValkyrieType for ValkyrieNDArray {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}
