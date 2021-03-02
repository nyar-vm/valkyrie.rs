use super::*;
use nyar_error::FileSpan;

/// A unique identifier used to query the valkyrie object
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieID {
    name: Vec<String>,
    file: FileSpan,
}

impl Debug for ValkyrieID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ValkyrieID").field(&self.name).finish()
    }
}

impl Display for ValkyrieID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name.join("∷"))
    }
}

impl ValkyrieID {
    pub fn new<I>(path: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self { name: path.into_iter().collect(), file: Default::default() }
    }
    pub fn name(&self) -> &str {
        match self.name.last() {
            Some(s) => s.as_str(),
            None => panic!("Empty namepath"),
        }
    }
    pub fn full_name(&self) -> &[String] {
        self.name.as_slice()
    }
    pub fn namespace(&self) -> &[String] {
        match self.name.len() {
            0 => panic!("Empty namepath"),
            1 => &[],
            _ => &self.name[0..self.name.len() - 1],
        }
    }
}
