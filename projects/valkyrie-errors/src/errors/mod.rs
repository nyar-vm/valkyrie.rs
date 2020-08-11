use std::{
    error::{Error, Report},
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use ariadne::{Report, ReportKind};

use crate::{parsing::SyntaxError, runtime::RuntimeError, DuplicateError, FileID, RuntimeError};

pub mod display;

pub type ValkyrieReport = Report<(FileID, Range<usize>)>;

pub type ValkyrieResult<T = ()> = Result<T, ValkyrieErrorKind>;

pub trait ValkyrieError: Error {
    fn box_error(self) -> ValkyrieErrorKind;

    fn error_code(&self) -> usize;
}

impl Error for ValkyrieErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ValkyrieErrorKind::Duplicate(e) => Some(e.as_ref()),
            ValkyrieErrorKind::Runtime(e) => Some(e.as_ref()),
            ValkyrieErrorKind::Parsing(e) => Some(e.as_ref()),
        }
    }
}

impl Debug for ValkyrieErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieErrorKind::Duplicate(e) => Debug::fmt(e, f),
            ValkyrieErrorKind::Runtime(e) => Debug::fmt(e, f),
            ValkyrieErrorKind::Parsing(e) => Debug::fmt(e, f),
        }
    }
}

impl Display for ValkyrieErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieErrorKind::Duplicate(e) => Display::fmt(e, f),
            ValkyrieErrorKind::Runtime(e) => Display::fmt(e, f),
            ValkyrieErrorKind::Parsing(e) => Display::fmt(e, f),
        }
    }
}

impl ValkyrieError for ValkyrieErrorKind {
    fn box_error(self) -> ValkyrieErrorKind {
        self
    }

    fn error_code(&self) -> usize {
        match &self {
            ValkyrieErrorKind::Duplicate(e) => e.error_code(),
            ValkyrieErrorKind::Runtime(e) => e.error_code(),
            ValkyrieErrorKind::Parsing(e) => e.error_code(),
        }
    }
}

#[derive(Clone)]
pub enum ValkyrieErrorKind {
    Duplicate(Box<DuplicateError>),
    Runtime(Box<RuntimeError>),
    Parsing(Box<SyntaxError>),
}

impl ValkyrieErrorBox {
    pub fn set_file(&mut self, file: FileID) {
        match &mut self.kind {
            ValkyrieErrorKind::Duplicate(_) => {}
            ValkyrieErrorKind::Runtime(_) => {}
            ValkyrieErrorKind::Parsing(s) => {
                s.span.file = file;
            }
        }
    }

    pub fn as_report(&self) -> ValkyrieReport {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(e) => e.as_report(self.level),
            ValkyrieErrorKind::Runtime(e) => e.as_report(self.level),
            ValkyrieErrorKind::Parsing(e) => e.as_report(self.level),
        }
    }
}
