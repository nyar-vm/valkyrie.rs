use std::fmt::{Debug, Display, Formatter};

/// A unique identifier used to query the valkyrie object
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieID {
    path: Vec<String>,
}

impl Debug for ValkyrieID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ValkyrieID").field(&self.path).finish()
    }
}

impl Display for ValkyrieID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path.join("∷"))
    }
}

impl ValkyrieID {
    pub fn new<I>(path: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self { path: path.into_iter().collect() }
    }
    pub fn name(&self) -> &str {
        match self.path.last() {
            Some(s) => s.as_str(),
            None => panic!("Empty namepath"),
        }
    }
    pub fn full_name(&self) -> &[String] {
        self.path.as_slice()
    }
    pub fn namespace(&self) -> &[String] {
        match self.path.len() {
            0 => panic!("Empty namepath"),
            1 => &[],
            _ => &self.path[0..self.path.len() - 1],
        }
    }
}
