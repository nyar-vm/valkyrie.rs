use super::*;

// #[derive(Clone, Debug, Scan)]
pub struct ValkyrieNDArray {
    inner_bytes: Vec<u8>,
    data_type: NDArrayDataType,
    dimension: Vec<usize>,
}

unsafe impl GcSafe for ValkyrieNDArray {}
unsafe impl Scan for ValkyrieNDArray {
    fn scan(&self, _: &mut Scanner<'_>) {}
}
#[derive(Clone, Debug)]
pub enum NDArrayDataType {
    Float32,
    Float64,
}

impl ValkyrieNDArray {}

impl ValkyrieType for ValkyrieNDArray {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        todo!()
    }
}
