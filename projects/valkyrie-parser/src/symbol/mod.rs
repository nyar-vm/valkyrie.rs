use std::fmt::{Display, Formatter};
use std::ops::Range;

pub const KEYWORDS: [&str; 3] = ["true", "false", "null"];

#[derive(Clone, Debug)]
pub struct ValkyrieIdentifier {
    pub name: String,
    pub range: Range<usize>,
}

impl ValkyrieIdentifier {
    pub fn is_normal(&self) -> bool {
        self.name.starts_with(|c: char| c.is_ascii_lowercase())
    }
}

impl Display for ValkyrieIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}