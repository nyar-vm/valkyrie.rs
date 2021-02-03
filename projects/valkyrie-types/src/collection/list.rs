use super::*;
use serde::{ser::SerializeSeq, Serialize, Serializer};

#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct NyarTuple<T> {
    pub raw: Vector<T>,
}

impl<T> Serialize for NyarTuple<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.raw.len()))?;
        for element in self.raw.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

unsafe impl<T: GcSafe> GcSafe for NyarTuple<T> {}
unsafe impl<T: GcDrop> GcDrop for NyarTuple<T> {}
unsafe impl<T: Scan> Scan for NyarTuple<T> {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        self.raw.iter().for_each(|v| scanner.scan(v))
    }
}

impl<T> Debug for NyarTuple<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.raw.iter()).finish()
    }
}

impl<T, U> FromIterator<U> for NyarTuple<T>
where
    U: Into<T>,
{
    fn from_iter<I>(items: I) -> Self
    where
        I: IntoIterator<Item = U>,
    {
        let mut empty = NyarTuple::default();
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

impl<T> NyarTuple<T> {
    pub fn get(&self, ordinal: ValkyrieValue) -> ValkyrieResult<Option<ValkyrieValue>> {
        let index = ValkyrieOrdinal::try_from(ordinal)?.as_offset(self.raw.len());
        Ok(self.raw.get(index).cloned())
    }
    pub fn get_range(&self, head: ValkyrieValue, tail: ValkyrieValue, step: ValkyrieValue) -> ValkyrieResult<NyarTuple> {
        let head = ValkyrieOrdinal::try_from(head)?.as_offset(self.raw.len());
        let tail = ValkyrieOrdinal::try_from(tail)?.as_offset(self.raw.len());
        // 0 is prohibited, zero does not indicate duplication
        let step = ValkyrieOrdinal::try_from(step)?.ordinal;
        if step > 0 {
            Ok(NyarTuple { raw: self.raw.iter().take(tail).skip(head).step_by(step as usize).cloned().collect() })
        }
        else {
            Ok(NyarTuple { raw: self.raw.iter().rev().take(tail).skip(head).step_by(-step as usize).cloned().collect() })
        }
    }

    pub fn append_one<U: Into<T>>(&mut self, item: U) {
        self.raw.push_back(item.into())
    }
    pub fn append_many<U: Iterator<Item = T>>(&mut self, items: U) {
        for item in items {
            self.raw.push_back(item)
        }
    }
    pub fn prepend_one<U: Into<T>>(&mut self, item: U) {
        self.raw.push_front(item.into())
    }
    pub fn prepend_many<U: Iterator<Item = T>>(&mut self, items: U) {
        for item in items {
            self.raw.push_front(item)
        }
    }
}
