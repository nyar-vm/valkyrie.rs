#![feature(error_reporter)]

mod duplicates;
mod errors;
mod runtime_error;
mod syntax_error;

#[cfg(feature = "url")]
pub use url::Url;

#[cfg(feature = "pex")]
pub use pex::ParseState;

#[cfg(feature = "serde_json")]
pub use serde_json::Value as JsonValue;

pub use crate::{
    duplicates::{DuplicateError, DuplicateKind},
    errors::{ValkyrieError, ValkyrieErrorType, ValkyrieResult},
    runtime_error::RuntimeError,
    syntax_error::SyntaxError,
};
