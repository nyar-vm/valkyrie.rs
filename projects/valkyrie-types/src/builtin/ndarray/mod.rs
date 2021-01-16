use super::*;

use ndarray::{Array, Array2};

#[derive(Clone, Debug, Scan)]
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

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        todo!()
    }
}
