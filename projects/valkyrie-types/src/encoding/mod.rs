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
