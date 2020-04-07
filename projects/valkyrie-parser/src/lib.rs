use ops::Range;
use std::ops;
use std::ops::Deref;

use valkyrie_errors::{FileID, ValkyrieError};

mod parser;

#[derive(Debug, Default)]
pub struct ValkyrieParser {
    file: FileID,
    errors: Vec<ValkyrieError>,
}
