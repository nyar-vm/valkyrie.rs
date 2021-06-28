use super::*;
use crate::{ValkyrieError, ValkyrieResult};
use nyar_collection::NyarTuple;
use nyar_number::ToPrimitive;

pub type ValkyrieList = NyarTuple<ValkyrieValue>;

pub struct ValkyrieOrdinal {
    ordinal: isize,
}

impl TryFrom<ValkyrieValue> for ValkyrieOrdinal {
    type Error = ValkyrieError;

    fn try_from(value: ValkyrieValue) -> Result<Self, Self::Error> {
        match value {
            ValkyrieValue::Number(v) => match v.to_isize() {
                Some(0) => Err(ValkyrieError::custom("ordinal must not zero")),
                Some(s) => Ok(Self { ordinal: s }),
                None => Err(ValkyrieError::custom("ordinal must integer")),
            },
            _ => Err(ValkyrieError::custom("ordinal must number")),
        }
    }
}

impl ValkyrieOrdinal {
    pub fn as_offset(&self, count: usize) -> usize {
        if self.ordinal > 0 { (self.ordinal - 1) as usize } else { count - (self.ordinal.abs() as usize) }
    }
}

impl ValkyrieValue {
    pub fn as_index(&self) -> ValkyrieResult<isize> {
        match self {
            ValkyrieValue::Number(v) => match v.to_isize() {
                Some(s) => Ok(s),
                None => Err(ValkyrieError::custom("index must integer")),
            },
            _ => Err(ValkyrieError::custom("index must number")),
        }
    }
}
