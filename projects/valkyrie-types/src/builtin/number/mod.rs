use crate::values::{ValkyrieType, ValkyrieValue, ValkyrieValueType};
use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ValkyrieNumber {
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Float32(OrderedFloat<f32>),
    Float64(OrderedFloat<f64>),
}

impl ValkyrieValueType for ValkyrieNumber {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(*self)
    }
    fn as_type(&self) -> ValkyrieType {
        match self {
            Self::Unsigned8(_) => ValkyrieType::Integer { sign: false, bits: 8 },
            Self::Unsigned16(_) => ValkyrieType::Integer { sign: false, bits: 16 },
            Self::Unsigned32(_) => ValkyrieType::Integer { sign: false, bits: 32 },
            Self::Unsigned64(_) => ValkyrieType::Integer { sign: false, bits: 64 },
            Self::Integer8(_) => ValkyrieType::Integer { sign: true, bits: 8 },
            Self::Integer16(_) => ValkyrieType::Integer { sign: true, bits: 16 },
            Self::Integer32(_) => ValkyrieType::Integer { sign: true, bits: 32 },
            Self::Integer64(_) => ValkyrieType::Integer { sign: true, bits: 64 },
            Self::Float32(_) => ValkyrieType::Decimal { float: true, bits: 32 },
            Self::Float64(_) => ValkyrieType::Decimal { float: true, bits: 64 },
        }
    }
}

impl ValkyrieValueType for u8 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Unsigned8(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: false, bits: 8 }
    }
}

impl ValkyrieValueType for u16 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Unsigned16(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: false, bits: 16 }
    }
}

impl ValkyrieValueType for u32 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Unsigned32(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: false, bits: 32 }
    }
}

impl ValkyrieValueType for u64 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Unsigned64(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: false, bits: 64 }
    }
}

impl ValkyrieValueType for i8 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Integer8(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: true, bits: 8 }
    }
}

impl ValkyrieValueType for i16 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Integer16(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: true, bits: 16 }
    }
}

impl ValkyrieValueType for i32 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Integer32(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: true, bits: 32 }
    }
}

impl ValkyrieValueType for i64 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Integer64(*self))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Integer { sign: true, bits: 64 }
    }
}
impl ValkyrieValueType for f32 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Float32(OrderedFloat(*self)))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Decimal { float: true, bits: 32 }
    }
}

impl ValkyrieValueType for f64 {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Number(ValkyrieNumber::Float64(OrderedFloat(*self)))
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Decimal { float: true, bits: 64 }
    }
}
