use std::num::{ParseFloatError, ParseIntError};

use crate::{errors::ValkyrieErrorLevel, ValkyrieError, ValkyrieErrorKind};
#[cfg(feature = "pex")]
use pex::StopBecause;

#[cfg(feature = "pex")]
impl From<StopBecause> for ValkyrieError {
    fn from(error: StopBecause) -> Self {
        ValkyrieError::syntax_error(error.to_string(), error.range())
    }
}

impl From<ParseFloatError> for ValkyrieError {
    fn from(error: ParseFloatError) -> Self {
        ValkyrieError::syntax_error(error.to_string(), 0..0)
    }
}

impl From<ParseIntError> for ValkyrieError {
    fn from(value: ParseIntError) -> Self {
        ValkyrieError::syntax_error(value.to_string(), 0..0)
    }
}
