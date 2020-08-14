use crate::{SyntaxError, ValkyrieError};
use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

impl<T> From<T> for ValkyrieError
where
    T: Into<SyntaxError>,
{
    fn from(value: T) -> Self {
        ValkyrieError::Syntax(Box::new(value.into()))
    }
}

impl From<ParseBoolError> for SyntaxError {
    fn from(value: ParseBoolError) -> Self {
        SyntaxError::new(value)
    }
}
impl From<ParseIntError> for SyntaxError {
    fn from(value: ParseIntError) -> Self {
        SyntaxError::new(value)
    }
}

impl From<ParseFloatError> for SyntaxError {
    fn from(value: ParseFloatError) -> Self {
        SyntaxError::new(value)
    }
}
