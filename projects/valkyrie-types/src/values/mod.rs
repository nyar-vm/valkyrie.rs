// mod der;
// mod ser;

use crate::ValkyrieNumber;
use std::rc::Rc;
use wasmtime::component::{types, types::OptionType, Type, Val};

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
    Result(Rc<MaybeType>),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ValkyrieType {
    Boolean,
    SmallDecimal { float: bool, bits: u32 },
    SmallInteger { sign: bool, bits: u32 },
    Text { character: bool, encoding: String },
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
            ValkyrieValue::Number(_) => {
                todo!()
            }
            ValkyrieValue::Result(_) => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MaybeType {
    left: bool,
    value: ValkyrieValue,
}

impl MaybeType {
    pub fn some<V>(v: V) -> MaybeType
    where
        V: ValkyrieValueType,
    {
        Self { left: false, value: ValkyrieValue::Nothing }
    }
    pub fn error<V>(v: V) -> MaybeType
    where
        V: ValkyrieValueType,
    {
        todo!()
    }
    pub fn none(ty: &Type) -> MaybeType {
        todo!()
    }
    pub fn null() -> MaybeType {
        todo!()
    }
    pub fn never() -> MaybeType {
        todo!()
    }
}

impl ValkyrieValueType for MaybeType {
    fn as_valkyrie(&self) -> ValkyrieValue {
        self.clone().to_valkyrie()
    }
    fn to_valkyrie(self) -> ValkyrieValue {
        ValkyrieValue::Result(Rc::new(self))
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
            Val::Char(_) => {
                todo!()
            }
            Val::String(_) => {
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
            Val::Option(v) => todo!(),
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
            Type::S8 => ValkyrieType::SmallInteger { sign: true, bits: 8 },
            Type::U8 => ValkyrieType::SmallInteger { sign: false, bits: 8 },
            Type::S16 => ValkyrieType::SmallInteger { sign: true, bits: 16 },
            Type::U16 => ValkyrieType::SmallInteger { sign: false, bits: 16 },
            Type::S32 => ValkyrieType::SmallInteger { sign: true, bits: 32 },
            Type::U32 => ValkyrieType::SmallInteger { sign: false, bits: 32 },
            Type::S64 => ValkyrieType::SmallInteger { sign: true, bits: 64 },
            Type::U64 => ValkyrieType::SmallInteger { sign: false, bits: 64 },
            Type::Float32 => ValkyrieType::SmallDecimal { bits: 32 },
            Type::Float64 => ValkyrieType::SmallDecimal { bits: 64 },
            Type::Char => {
                todo!()
            }
            Type::String => {
                todo!()
            }
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
