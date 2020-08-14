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
    pub info: String,
    pub span: FileID,
    pub range: Range<u32>,
    pub level: DiagnosticLevel,
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl SyntaxError {
    pub fn new<T: ToString>(message: T) -> Self {
        Self { info: message.to_string(), span: Default::default(), range: Default::default(), level: DiagnosticLevel::Error }
    }
}
