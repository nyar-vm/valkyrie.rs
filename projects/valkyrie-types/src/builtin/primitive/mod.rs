use shredder::Gc;

use crate::{types::ValkyrieMetaType, utils::primitive_type, ValkyrieError, ValkyrieNumber, ValkyrieType, ValkyrieValue};

impl From<char> for ValkyrieValue {
    fn from(value: char) -> Self {
        Self::Unicode(value)
    }
}

impl From<u8> for ValkyrieValue {
    fn from(value: u8) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}

impl ValkyrieValue {
    pub fn parse_integer(input: &str, radix: u32) -> Result<ValkyrieValue, ValkyrieError> {
        Ok(Self::Number(ValkyrieNumber::parse_integer_radix(input, radix).expect("")))
    }
    pub fn parse_decimal(input: &str, radix: u32) -> Result<ValkyrieValue, ValkyrieError> {
        Ok(Self::Number(ValkyrieNumber::parse_decimal_radix(input, radix).expect("")))
    }
}

impl ValkyrieType for u8 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }

    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned8")
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned8")
    }
}

impl From<u16> for ValkyrieValue {
    fn from(value: u16) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}

impl ValkyrieType for u16 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }

    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned16")
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned16")
    }
}

impl From<u32> for ValkyrieValue {
    fn from(value: u32) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}

impl ValkyrieType for u32 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }

    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned32")
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned32")
    }
}

impl From<u64> for ValkyrieValue {
    fn from(value: u64) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}

impl ValkyrieType for u64 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }

    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned64")
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned64")
    }
}
impl From<u128> for ValkyrieValue {
    fn from(value: u128) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}

impl ValkyrieType for u128 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }

    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned128")
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Unsigned128")
    }
}
impl From<usize> for ValkyrieValue {
    fn from(value: usize) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}

impl ValkyrieType for usize {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }

    fn static_type() -> Gc<ValkyrieMetaType> {
        if cfg!(target_pointer_width = "64") {
            primitive_type("std.primitive.Unsigned64")
        }
        else {
            primitive_type("std.primitive.Unsigned32")
        }
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        Self::static_type()
    }
}
impl From<i8> for ValkyrieValue {
    fn from(value: i8) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}
impl ValkyrieType for i8 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }
    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Integer8")
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        Self::static_type()
    }
}
impl From<i16> for ValkyrieValue {
    fn from(value: i16) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}
impl ValkyrieType for i16 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }
    fn static_type() -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Integer16")
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        Self::static_type()
    }
}
impl From<i32> for ValkyrieValue {
    fn from(value: i32) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}
impl ValkyrieType for i32 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Integer32")
    }
}
impl From<i64> for ValkyrieValue {
    fn from(value: i64) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}
impl ValkyrieType for i64 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Integer64")
    }
}
impl From<i128> for ValkyrieValue {
    fn from(value: i128) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}
impl ValkyrieType for i128 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.primitive.Integer128")
    }
}
impl From<isize> for ValkyrieValue {
    fn from(value: isize) -> Self {
        ValkyrieValue::Number(ValkyrieNumber::from(value))
    }
}
impl ValkyrieType for isize {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::from(self)
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        if cfg!(target_pointer_width = "64") {
            primitive_type("std.primitive.Integer64")
        }
        else {
            primitive_type("std.primitive.Integer32")
        }
    }
}

impl ValkyrieType for f32 {
    fn boxed(self) -> ValkyrieValue {
        match ValkyrieNumber::try_from(self) {
            Ok(o) => ValkyrieValue::Number(o),
            Err(_) => {
                todo!()
            }
        }
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Float32");
        Gc::new(this)
    }
}

impl ValkyrieType for f64 {
    fn boxed(self) -> ValkyrieValue {
        match ValkyrieNumber::try_from(self) {
            Ok(o) => ValkyrieValue::Number(o),
            Err(_) => {
                todo!()
            }
        }
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Float64");
        Gc::new(this)
    }
}

impl ValkyrieType for char {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Unicode(self)
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        primitive_type("std.text.Unicode")
    }
}

impl ValkyrieType for String {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::UTF8String(Gc::new(self))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType>
    where
        Self: Sized,
    {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.text.UTF8Text");
        Gc::new(this)
    }
}
