use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};
use polars::prelude::LazyFrame;
use std::sync::Arc;

pub struct ValkyrieDataFrame {
    any_frame: LazyFrame,
}

impl ValkyrieType for ValkyrieDataFrame {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}
