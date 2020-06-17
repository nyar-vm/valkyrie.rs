use dashu::integer::IBig;
use std::sync::Arc;

use crate::{types::ValkyrieMetaType, utils::primitive_type, ValkyrieClass, ValkyrieType, ValkyrieValue};

use super::*;

impl ValkyrieType for u8 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_type() -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned8")
    }
    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned8")
    }
}

impl ValkyrieType for u16 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_type() -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned16")
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned16")
    }
}

impl ValkyrieType for u32 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_type() -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned32")
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned32")
    }
}

impl ValkyrieType for u64 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_type() -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned64")
    }
    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned64")
    }
}

impl ValkyrieType for u128 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_type() -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned128")
    }
    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned128")
    }
}

impl ValkyrieType for usize {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        if cfg!(target_pointer_width = "64") {
            primitive_type("std.primitive.Unsigned64")
        }
        else {
            primitive_type("std.primitive.Unsigned32")
        }
    }
}

impl ValkyrieType for i8 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_type() -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Integer8");
        Arc::new(this)
    }
    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        Self::static_type()
    }
}

impl ValkyrieType for i16 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_type() -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Integer16");
        Arc::new(this)
    }
    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        Self::static_type()
    }
}

impl ValkyrieType for i32 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Integer32");
        Arc::new(this)
    }
}

impl ValkyrieType for i64 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Integer64");
        Arc::new(this)
    }
}

impl ValkyrieType for i128 {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Integer128");
        Arc::new(this)
    }
}

impl ValkyrieType for isize {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Integer64");
        Arc::new(this)
    }
}

impl ValkyrieType for IBig {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Integer(self)
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.math.Integer");
        Arc::new(this)
    }
}

impl ValkyrieType for f32 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Float32(self)
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Float32");
        Arc::new(this)
    }
}

impl ValkyrieType for f64 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Float64(self)
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Float64");
        Arc::new(this)
    }
}

impl ValkyrieType for char {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::UTF8Character(self)
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.text.Unicode");
        Arc::new(this)
    }
}

impl ValkyrieType for String {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::UTF8String(Arc::new(self))
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType>
    where
        Self: Sized,
    {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.UTF8Text");
        Arc::new(this)
    }
}
