// mod der;
// mod ser;

use crate::ValkyrieNumber;
use std::rc::Rc;
use wasmtime::component::{types, types::OptionType, Type, Val};

pub trait ValkyrieValue {
    fn as_valkyrie(&self) -> ValkyrieValueType;
    fn to_valkyrie(self) -> ValkyrieValueType {
        self.as_valkyrie()
    }
}
pub trait ValkyrieType {
    fn as_valkyrie(&self) -> ValkyrieValueType;
    fn to_valkyrie(self) -> ValkyrieValueType {
        self.as_valkyrie()
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ValkyrieValueType {
    /// ADT = -1
    Nothing,
    /// An uninitialized value, null for pointer types, and default for value types
    ///
    /// Trying to read from an uninitialized value is a fatal error.
    Unit,
    /// ADT = 2
    ///
    /// Native boolean type, 8bit
    Boolean(Option<bool>),
    Number(ValkyrieNumber),
    Result(Rc<MaybeType>),
}

impl ValkyrieValue for ValkyrieValueType {
    fn as_valkyrie(&self) -> ValkyrieValueType {
        self.clone()
    }
    fn to_valkyrie(self) -> ValkyrieValueType {
        self
    }
    fn as_type(&self) -> ValkyrieType {
        match self {
            ValkyrieValueType::Nothing => {
                todo!()
            }
            ValkyrieValueType::Unit => {
                todo!()
            }
            ValkyrieValueType::Boolean(_) => {
                todo!()
            }
            ValkyrieValueType::Number(_) => {
                todo!()
            }
            ValkyrieValueType::Result(_) => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MaybeType {
    left: bool,
    value: ValkyrieValueType,
}

impl MaybeType {
    pub fn some<V>(v: V) -> MaybeType
    where
        V: ValkyrieValue,
    {
        Self { left: false, value: ValkyrieValueType::Nothing }
    }
    pub fn error<V>(v: V) -> MaybeType
    where
        V: ValkyrieValue,
    {
        Self::Right(Box::new(v.to_valkyrie()))
    }
    pub fn none(ty: &OptionType) -> MaybeType {
        match ty.ty() {
            Type::Bool => {
                todo!()
            }
            Type::S8 => {
                todo!()
            }
            Type::U8 => {
                todo!()
            }
            Type::S16 => {
                todo!()
            }
            Type::U16 => {
                todo!()
            }
            Type::S32 => {
                todo!()
            }
            Type::U32 => {
                todo!()
            }
            Type::S64 => {
                todo!()
            }
            Type::U64 => {
                todo!()
            }
            Type::Float32 => {
                todo!()
            }
            Type::Float64 => {
                todo!()
            }
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
    pub fn null() -> MaybeType {
        todo!()
    }
    pub fn never() -> MaybeType {
        todo!()
    }
}

impl ValkyrieValue for MaybeType {
    fn as_valkyrie(&self) -> ValkyrieValueType {
        self.clone().to_valkyrie()
    }
    fn to_valkyrie(self) -> ValkyrieValueType {
        ValkyrieValueType::Result(Rc::new(self))
    }
}

impl ValkyrieValue for Val {
    fn as_valkyrie(&self) -> ValkyrieValueType {
        match self {
            Val::Bool(v) => v.into(),
            Val::S8(v) => v.into(),
            Val::U8(v) => v.into(),
            Val::S16(v) => v.into(),
            Val::U16(v) => v.into(),
            Val::S32(v) => v.into(),
            Val::U32(v) => v.into(),
            Val::S64(v) => v.into(),
            Val::U64(v) => v.into(),
            Val::Float32(v) => v.into(),
            Val::Float64(v) => v.into(),
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
            Val::Option(v) => match v.value() {
                Some(s) => {}
                None => MaybeType::none(v.ty()).to_valkyrie(),
            },
            Val::Result(v) => match v.value() {
                Some(s) => {}
                None => MaybeType::none(v.ty()).to_valkyrie(),
            },
            Val::Flags(_) => {
                todo!()
            }
            Val::Resource(_) => {
                todo!()
            }
        }
    }
}
