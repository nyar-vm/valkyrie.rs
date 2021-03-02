// mod der;
// mod ser;

use crate::{ValkyrieMaybe, ValkyrieNumber, ValkyrieText};
use std::rc::Rc;

mod wasm_abi;

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
