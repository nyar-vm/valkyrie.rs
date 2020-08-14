use crate::ValkyrieError;
use serde_json::{Error, Value};
use std::sync::Arc;

impl From<Error> for ValkyrieError {
    fn from(value: Error) -> Self {
        ValkyrieError::custom(value.to_string())
    }
}
