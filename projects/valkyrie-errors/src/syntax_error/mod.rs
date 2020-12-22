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
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub info: String,
    pub span: DefinitionSpan,
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
        Self { info: message.to_string(), span: Default::default(), level: DiagnosticLevel::Error }
    }
    pub fn with_span(self, span: &Range<u32>) -> Self {
        let mut out = self.clone();
        out.span.range = span.clone();
        out
    }
    pub fn with_file(self, file: FileID) -> Self {
        let mut out = self.clone();
        out.span.file = file;
        out
    }
    pub fn with_level(self, level: DiagnosticLevel) -> Self {
        Self { level, ..self }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DefinitionSpan {
    /// File path or cell id
    file: FileID,
    /// Start offset and end offset
    range: Range<u32>,
}

impl DefinitionSpan {
    pub fn get_path<P>(&self, workspace: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let workspace = workspace.as_ref();
        let file = workspace.join(&self.file);
        file
    }
}

impl std::io::Write for DefinitionSpan {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
