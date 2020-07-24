use ndarray::{Array};

pub struct ValkyrieNDArray {
    inner_bytes: *mut u8,
    data_type: NDArrayDataType,
    dimension: Vec<usize>,
}

pub enum NDArrayDataType {
    Float32,
    Float64,
}

impl ValkyrieNDArray {
    pub fn recast<A, D>(&self) -> Array<A, D> {
        todo!()
    }
}
