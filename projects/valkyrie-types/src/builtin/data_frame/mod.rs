use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};
use polars::prelude::LazyFrame;
use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

#[derive(Clone)]
pub struct ValkyrieDataFrame {
    any_frame: LazyFrame,
}

impl Debug for ValkyrieDataFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ValkyrieType for ValkyrieDataFrame {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}
