use crate::{ValkyrieNumber, ValkyrieValue};

use nyar_error::RuntimeError;

/// implicit cast
/// explicit cast
/// implicit into
/// explicit cast
pub trait IntoValkyrie {
    fn as_valkyrie(&self) -> Result<ValkyrieValue, RuntimeError>;
}

pub trait FromValkyrie {
    fn as_rust<T>(&self) -> Result<T, RuntimeError>;
}

pub struct EncodeAny {}

pub struct EncodeList {
    items: Vec<ValkyrieValue>,
}

impl EncodeAny {
    pub fn encode_bool(self, v: bool) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Boolean(v))
    }

    pub fn encode_u8(self, v: u8) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_u16(self, v: u16) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_u32(self, v: u32) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_u64(self, v: u64) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_u128(self, v: u128) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_usize(self, v: usize) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_i8(self, v: i8) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_i16(self, v: i16) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_i32(self, v: i32) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_i64(self, v: i64) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_i128(self, v: i128) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_isize(self, v: isize) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Number(ValkyrieNumber::from(v)))
    }

    pub fn encode_unicode(self, v: char) -> Result<ValkyrieValue, RuntimeError> {
        Ok(ValkyrieValue::Unicode(v))
    }

    pub fn encode_some<T: IntoValkyrie>(self, v: ValkyrieValue) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_none(self) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_success<T: IntoValkyrie>(self, v: ValkyrieValue) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_failure<T: IntoValkyrie>(self, v: ValkyrieValue) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_unit(self) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_unit_structure(self, name: &'static str) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_unit_variant(self, name: &'static str, variant_index: u32) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_unit_flags(self, name: &'static str, flags: u32) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }

    pub fn encode_list(self, capacity: usize) -> Result<EncodeList, RuntimeError> {
        Ok(EncodeList { items: Vec::with_capacity(capacity) })
    }
}

impl EncodeList {
    pub fn encode_element<T>(&mut self, value: &T) -> Result<(), RuntimeError>
    where
        T: IntoValkyrie + ?Sized,
    {
        self.items.push(value.as_valkyrie()?);
        Ok(())
    }

    pub fn finish(self) -> Result<ValkyrieValue, RuntimeError> {
        todo!()
    }
}
