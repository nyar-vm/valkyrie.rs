use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    num::ParseIntError,
    ops::Range,
};

pub type ValkyrieResult<T> = Result<T, ValkyrieError>;

pub struct ValkyrieError {
    kind: Box<ValkyrieErrorKind>,
}

impl ValkyrieError {
    pub fn custom<S: ToString>(message: S) -> Self {
        Self { kind: Box::new(ValkyrieErrorKind::Custom(message.to_string())) }
    }
    pub fn syntax_error<S: ToString>(message: S, range: Range<usize>) -> Self {
        Self { kind: Box::new(ValkyrieErrorKind::SyntaxError { message: message.to_string(), range }) }
    }
}

pub enum ValkyrieErrorKind {
    Custom(String),
    SyntaxError { message: String, range: Range<usize> },
}

impl Error for ValkyrieError {}

impl Debug for ValkyrieErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ValkyrieErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ValkyrieErrorKind {}

impl Debug for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<ParseIntError> for ValkyrieError {
    fn from(value: ParseIntError) -> Self {
        Self { kind: Box::new(ValkyrieErrorKind::SyntaxError { message: value.to_string(), range: 0..0 }) }
    }
}
