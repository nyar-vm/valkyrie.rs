// mod der;
// mod ser;

use crate::{ValkyrieMaybe, ValkyrieNumber, ValkyrieText};
use std::rc::Rc;
use wasmtime::component::{types::OptionType, Type, Val};

pub trait ValkyrieValueType {
    fn as_valkyrie(&self) -> ValkyrieValue;
    fn to_valkyrie(self) -> ValkyrieValue
    where
        Self: Sized,
    {
        self.as_valkyrie()
    }
    fn as_type(&self) -> ValkyrieType {
        self.as_valkyrie().as_type()
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ValkyrieValue {
    /// ADT = -1
    Nothing,
    /// An uninitialized value, null for pointer types, and default for value types
    ///
    /// Trying to read from an uninitialized value is a fatal error.
    Unit,
    /// ADT = 2
    ///
    /// Native boolean type, 8bit
    Boolean(bool),
    Number(ValkyrieNumber),
    Text(ValkyrieText),
    Maybe(Box<ValkyrieMaybe>),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ValkyrieType {
    Boolean,
    Decimal { float: bool, bits: u32 },
    Integer { sign: bool, bits: u32 },
    Text { character: bool, encoding: &'static str },
    Literal(Box<ValkyrieValue>),
}

impl ValkyrieValueType for ValkyrieValue {
    fn as_valkyrie(&self) -> ValkyrieValue {
        self.clone()
    }
    fn to_valkyrie(self) -> ValkyrieValue {
        self
    }
    fn as_type(&self) -> ValkyrieType {
        match self {
            ValkyrieValue::Nothing => {
                todo!()
            }
            ValkyrieValue::Unit => {
                todo!()
            }
            ValkyrieValue::Boolean(_) => {
                todo!()
            }
            ValkyrieValue::Number(v) => v.as_type(),
            ValkyrieValue::Text(v) => v.as_type(),
            ValkyrieValue::Maybe(_) => {
                todo!()
            }
        }
    }
}

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
            Val::String(v) => {
                todo!()
            }
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
            Val::Option(v) => match v.value() {
                Some(s) => ValkyrieMaybe::some(s).as_valkyrie(),
                None => ValkyrieMaybe::none(v.ty().ty().as_type()).as_valkyrie(),
            },
            Val::Result(v) => todo!(),
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
