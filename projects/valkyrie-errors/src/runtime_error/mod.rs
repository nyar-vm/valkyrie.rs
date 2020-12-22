use crate::ValkyrieError;
use diagnostic::DiagnosticLevel;
use std::fmt::{Display, Formatter};

#[cfg(feature = "dashu")]
mod for_dashu;

#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub info: String,
    pub level: DiagnosticLevel,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl RuntimeError {
    pub fn new<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self { info: message.into(), level: DiagnosticLevel::Error }
    }
}

impl From<RuntimeError> for ValkyrieError {
    fn from(value: RuntimeError) -> Self {
        Self::Runtime(Box::new(value))
    }
}
