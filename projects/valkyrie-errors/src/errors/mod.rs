use crate::duplicates::DuplicateError;

use crate::SyntaxError;
use std::{
    error::{Error, Report},
    fmt::{Debug, Display, Formatter},
};

pub mod display;

pub type ValkyrieResult<T = ()> = Result<T, ValkyrieError>;

pub trait ValkyrieErrorType: Error {
    fn boxed(self) -> ValkyrieError;
    fn error_code(&self) -> usize;
    fn as_report(&self) -> Report;
}

impl Error for ValkyrieError {}

impl Debug for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieError::Duplicate(v) => Debug::fmt(v, f),
            ValkyrieError::Custom(v) => Debug::fmt(v, f),
            ValkyrieError::Syntax(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieError::Duplicate(v) => Display::fmt(v, f),
            ValkyrieError::Custom(v) => Display::fmt(v, f),
            ValkyrieError::Syntax(v) => Display::fmt(v, f),
        }
    }
}

pub enum ValkyrieError {
    Syntax(Box<SyntaxError>),
    Duplicate(Box<DuplicateError>),
    Custom(Box<String>),
}

impl ValkyrieError {
    pub fn custom<S: ToString>(message: S) -> Self {
        Self::Custom(Box::new(message.to_string()))
    }
}
