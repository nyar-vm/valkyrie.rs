use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
};
mod for_parsing;
pub type ValkyrieResult<T> = Result<T, ValkyrieError>;

#[derive(Debug)]
pub struct ValkyrieError {
    kind: Box<ValkyrieErrorKind>,
    level: ValkyrieErrorLevel,
}

impl ValkyrieError {
    pub fn is_fatal(&self) -> bool {
        self.level == ValkyrieErrorLevel::Fatal
    }
}

#[derive(Debug)]
pub enum ValkyrieErrorKind {
    Custom(String),
    CompileError { code: usize, message: String, range: Range<usize> },
    SyntaxError { message: String, range: Range<usize> },
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ValkyrieErrorLevel {
    Trace = 0,
    Fatal = 255,
}

impl ValkyrieError {
    pub fn custom<S: ToString>(message: S) -> Self {
        Self { kind: Box::new(ValkyrieErrorKind::Custom(message.to_string())), level: ValkyrieErrorLevel::Trace }
    }
    pub fn compile_error<S: ToString>(code: usize, message: S, range: Range<usize>) -> Self {
        Self {
            kind: Box::new(ValkyrieErrorKind::CompileError { code, message: message.to_string(), range }),
            level: ValkyrieErrorLevel::Trace,
        }
    }

    pub fn syntax_error<S: ToString>(message: S, range: Range<usize>) -> Self {
        Self {
            kind: Box::new(ValkyrieErrorKind::SyntaxError { message: message.to_string(), range }),
            level: ValkyrieErrorLevel::Trace,
        }
    }
}

impl Error for ValkyrieError {}
impl Error for ValkyrieErrorKind {}

impl Display for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for ValkyrieErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ValkyrieErrorKind::Custom(message) => write!(f, "{}", message),
            ValkyrieErrorKind::SyntaxError { message, range } => write!(f, "{}", message),
            ValkyrieErrorKind::CompileError { code, message, range } => {
                write!(f, "Compile Error: {} (code: {})", message, code)
            }
        }
    }
}
