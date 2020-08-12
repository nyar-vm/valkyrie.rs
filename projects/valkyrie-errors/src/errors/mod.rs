use crate::duplicates::DuplicateError;

use std::error::{Error, Report};

pub mod display;

pub type ValkyrieResult<T = ()> = Result<T, ValkyrieError>;

pub trait ValkyrieErrorType: Error {
    fn boxed(self) -> ValkyrieError;
    fn error_code(&self) -> usize;
    fn as_report(&self) -> Report;
}

pub enum ValkyrieError {
    // Syntax(SyntaxError),
    Duplicate(Box<DuplicateError>),
    Custom(Box<dyn Error>),
}
