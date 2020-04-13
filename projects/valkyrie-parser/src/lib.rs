#![feature(once_cell)]

use valkyrie_errors::{FileID, ValkyrieError};

mod parser;
pub mod utils;

#[derive(Debug, Default)]
pub struct ValkyrieParser {
    file: FileID,
    errors: Vec<ValkyrieError>,
}
