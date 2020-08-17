use crate::SyntaxError;
use json5::Error;

impl From<Error> for SyntaxError {
    fn from(value: Error) -> Self {
        SyntaxError::new(value.to_string())
    }
}
