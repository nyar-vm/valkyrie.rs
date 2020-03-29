#![feature(box_syntax)]
#![feature(int_error_matching)]
#![feature(trivial_bounds)]

mod errors;
mod span;

pub use self::{
    errors::{NyarError, NyarErrorKind, Result},
    span::Span,
};
