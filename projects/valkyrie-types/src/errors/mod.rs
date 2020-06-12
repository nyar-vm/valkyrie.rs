use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    num::ParseIntError,
    ops::Range,
};
mod for_parsing;
pub type ValkyrieResult<T> = Result<T, ValkyrieError>;

#[derive(Debug)]
pub struct ValkyrieError {
    kind: Box<ValkyrieErrorKind>,
    level: ValkyrieErrorLevel,
}

#[derive(Debug)]
pub enum ValkyrieErrorKind {
    Custom(String),
    SyntaxError { message: String, range: Range<usize> },
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ValkyrieErrorLevel {
    Trace = 0,
}

impl ValkyrieError {
    pub fn custom<S: ToString>(message: S) -> Self {
        Self { kind: Box::new(ValkyrieErrorKind::Custom(message.to_string())), level: ValkyrieErrorLevel::Trace }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for ValkyrieErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieErrorKind::Custom(message) => write!(f, "{}", message),
            ValkyrieErrorKind::SyntaxError { message, range } => write!(f, "{}", message),
        }
    }
}

impl From<ParseIntError> for ValkyrieError {
    fn from(value: ParseIntError) -> Self {
        Self {
            kind: Box::new(ValkyrieErrorKind::SyntaxError { message: value.to_string(), range: 0..0 }),
            level: ValkyrieErrorLevel::Trace,
        }
    }
}
