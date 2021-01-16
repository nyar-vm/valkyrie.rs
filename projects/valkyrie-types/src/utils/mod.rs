use crate::types::ValkyrieMetaType;
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Debug, Formatter},
    slice::Iter,
    str::FromStr,
    sync::Arc,
};
use valkyrie_ast::NamePathNode;

pub fn primitive_type(name: &str) -> Gc<ValkyrieMetaType> {
    let mut this = ValkyrieMetaType::default();
    this.set_namepath(name);
    Gc::new(this)
}

#[derive(Clone)]
pub struct Namepath {
    path: Vec<Box<str>>,
}

unsafe impl GcSafe for Namepath {}
unsafe impl GcDrop for Namepath {}
unsafe impl Scan for Namepath {
    fn scan(&self, _: &mut Scanner<'_>) {}
}

impl Namepath {
    pub fn push(&mut self, path: &str) {
        self.path.push(Box::from(path))
    }
}

impl Debug for Namepath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut items = self.path.iter();
        match items.next() {
            Some(s) => {
                f.write_str(s)?;
            }
            None => f.write_str("<EMPTY>"),
        }
        for item in items {
            f.write_str("::")?;
            f.write_str(item)?;
        }
        Ok(())
    }
}

impl Default for Namepath {
    fn default() -> Self {
        Self { path: vec![] }
    }
}

impl FromStr for Namepath {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { path: s.split('.').map(|s| s.to_string()).collect() })
    }
}
