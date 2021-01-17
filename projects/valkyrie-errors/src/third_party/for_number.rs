use crate::{SyntaxError, ValkyrieError};
use nyar_number::NyarNumberError;

impl From<NyarNumberError> for ValkyrieError {
    fn from(error: NyarNumberError) -> Self {
        match error {
            NyarNumberError::ParseError(v) => ValkyrieError::Syntax(Box::new(SyntaxError::new(v))),
        }
    }
}
