#![feature(error_reporter)]

mod duplicates;
mod errors;
mod parsing;
mod runtime;

mod third_party;

#[cfg(feature = "url")]
pub use url::Url;

#[cfg(feature = "serde_json")]
pub use serde_json::Value as JsonValue;

pub use crate::errors::{ValkyrieError, ValkyrieErrorKind, ValkyrieReport, ValkyrieResult};
