use crate::ValkyrieValue;
use im::HashMap;
use serde::{ser::SerializeMap, Serialize, Serializer};
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::fmt::{Debug, Formatter};

pub mod dict;
pub mod list;
