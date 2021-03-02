use super::*;
use wasmtime::component::{
    types::{OptionType, ResultType},
    OptionVal, ResultVal, Type, Val,
};
mod maybe;

impl ValkyrieValueType for Val {
    fn as_valkyrie(&self) -> ValkyrieValue {
        match self {
            Val::Bool(v) => v.as_valkyrie(),
            Val::S8(v) => v.as_valkyrie(),
            Val::U8(v) => v.as_valkyrie(),
            Val::S16(v) => v.as_valkyrie(),
            Val::U16(v) => v.as_valkyrie(),
            Val::S32(v) => v.as_valkyrie(),
            Val::U32(v) => v.as_valkyrie(),
            Val::S64(v) => v.as_valkyrie(),
            Val::U64(v) => v.as_valkyrie(),
            Val::Float32(v) => v.as_valkyrie(),
            Val::Float64(v) => v.as_valkyrie(),
            Val::Char(v) => v.as_valkyrie(),
            Val::String(v) => v.as_ref().as_valkyrie(),
            Val::List(_) => {
                todo!()
            }
            Val::Record(_) => {
                todo!()
            }
            Val::Tuple(_) => {
                todo!()
            }
            Val::Variant(_) => {
                todo!()
            }
            Val::Enum(_) => {
                todo!()
            }
            Val::Option(v) => v.as_valkyrie(),
            Val::Result(v) => v.as_valkyrie(),
            Val::Flags(_) => {
                todo!()
            }
            Val::Resource(_) => {
                todo!()
            }
        }
    }
}

impl ValkyrieValueType for Type {
    fn as_valkyrie(&self) -> ValkyrieValue {
        unreachable!()
    }
    fn as_type(&self) -> ValkyrieType {
        match self {
            Type::Bool => ValkyrieType::Boolean,
            Type::S8 => ValkyrieType::Integer { sign: true, bits: 8 },
            Type::U8 => ValkyrieType::Integer { sign: false, bits: 8 },
            Type::S16 => ValkyrieType::Integer { sign: true, bits: 16 },
            Type::U16 => ValkyrieType::Integer { sign: false, bits: 16 },
            Type::S32 => ValkyrieType::Integer { sign: true, bits: 32 },
            Type::U32 => ValkyrieType::Integer { sign: false, bits: 32 },
            Type::S64 => ValkyrieType::Integer { sign: true, bits: 64 },
            Type::U64 => ValkyrieType::Integer { sign: false, bits: 64 },
            Type::Float32 => ValkyrieType::Decimal { float: true, bits: 32 },
            Type::Float64 => ValkyrieType::Decimal { float: true, bits: 64 },
            Type::Char => ValkyrieType::Text { character: true, encoding: "Unicode" },
            Type::String => ValkyrieType::Text { character: false, encoding: "Utf8Text" },
            Type::List(_) => {
                todo!()
            }
            Type::Record(_) => {
                todo!()
            }
            Type::Tuple(_) => {
                todo!()
            }
            Type::Variant(_) => {
                todo!()
            }
            Type::Enum(_) => {
                todo!()
            }
            Type::Option(_) => {
                todo!()
            }
            Type::Result(_) => {
                todo!()
            }
            Type::Flags(_) => {
                todo!()
            }
            Type::Own(_) => {
                todo!()
            }
            Type::Borrow(_) => {
                todo!()
            }
        }
    }
}
