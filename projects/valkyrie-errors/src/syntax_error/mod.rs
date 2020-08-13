use diagnostic::{DiagnosticLevel, FileID};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub info: String,
    pub span: FileID,
    pub range: Range<u32>,
    pub level: DiagnosticLevel,
}
