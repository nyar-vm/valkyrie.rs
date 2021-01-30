use crate::{encoding::IntoValkyrie, ValkyrieValue};
use im::Vector;
use itertools::iterate;
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::fmt::{Debug, Formatter};
use valkyrie_error::{
    third_party::{NyarReal, ToPrimitive},
    ValkyrieError, ValkyrieResult,
};

pub mod dict;
pub mod list;
