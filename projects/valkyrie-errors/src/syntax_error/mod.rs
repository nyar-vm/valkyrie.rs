#[cfg(feature = "json5")]
mod for_json5;
#[cfg(feature = "pex")]
mod for_pex;
#[cfg(feature = "serde_json")]
mod for_serde_json;

mod for_std;

use diagnostic::{DiagnosticLevel, FileID};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Range,
};

#[derive(Clone, Debug)]
pub struct SyntaxError {
    info: String,
    file: FileID,
    span: Range<u32>,
    level: DiagnosticLevel,
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl SyntaxError {
    pub fn new<T: ToString>(message: T) -> Self {
        Self { info: message.to_string(), file: Default::default(), span: Default::default(), level: DiagnosticLevel::Error }
    }
    pub fn with_span(self, span: &Range<u32>) -> Self {
        Self { span: span.clone(), ..self }
    }
    pub fn with_file(self, file: FileID) -> Self {
        Self { file, ..self }
    }
    pub fn with_level(self, level: DiagnosticLevel) -> Self {
        Self { level, ..self }
    }
}
