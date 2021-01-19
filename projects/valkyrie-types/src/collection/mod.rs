use crate::{encoding::IntoValkyrie, ValkyrieValue};
use im::Vector;
use itertools::iterate;
use std::fmt::{Debug, Formatter};
use valkyrie_error::{
    third_party::{NyarReal, ToPrimitive},
    ValkyrieError, ValkyrieResult,
};

#[derive(Default)]
pub struct ValkyrieList {
    pub raw: Vector<ValkyrieValue>,
}

impl Debug for ValkyrieList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.raw.iter()).finish()
    }
}

impl<T> FromIterator<T> for ValkyrieList
where
    T: Into<ValkyrieValue>,
{
    fn from_iter<I>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut empty = ValkyrieList::default();
        for item in items.into_iter() {
            empty.raw.push_back(item.into());
        }
        empty
    }
}

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

impl ValkyrieList {
    pub fn get(&self, ordinal: ValkyrieValue) -> ValkyrieResult<Option<ValkyrieValue>> {
        let index = ValkyrieOrdinal::try_from(ordinal)?.as_offset(self.raw.len());
        Ok(self.raw.get(index).cloned())
    }
    pub fn get_range(&self, head: ValkyrieValue, tail: ValkyrieValue, step: ValkyrieValue) -> ValkyrieResult<ValkyrieList> {
        let head = ValkyrieOrdinal::try_from(head)?.as_offset(self.raw.len());
        let tail = ValkyrieOrdinal::try_from(tail)?.as_offset(self.raw.len());
        // 0 is prohibited, zero does not indicate duplication
        let step = ValkyrieOrdinal::try_from(step)?.ordinal;
        if step > 0 {
            Ok(ValkyrieList { raw: self.raw.iter().take(tail).skip(head).step_by(step as usize).cloned().collect() })
        }
        else {
            Ok(ValkyrieList { raw: self.raw.iter().rev().take(tail).skip(head).step_by(-step as usize).cloned().collect() })
        }
    }

    pub fn append_one<T: Into<ValkyrieValue>>(&mut self, item: T) {
        self.raw.push_back(item.into())
    }
    pub fn append_many<I: Iterator<Item = ValkyrieValue>>(&mut self, items: I) {
        for item in items {
            self.raw.push_back(item)
        }
    }
    pub fn prepend_one<T: Into<ValkyrieValue>>(&mut self, item: T) {
        self.raw.push_front(item.into())
    }
    pub fn prepend_many<I: Iterator<Item = ValkyrieValue>>(&mut self, items: I) {
        for item in items {
            self.raw.push_front(item)
        }
    }
}
