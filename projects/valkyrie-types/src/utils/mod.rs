use crate::types::ValkyrieMetaType;
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Debug, Formatter, Write},
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
    pub fn length(&self) -> usize {
        self.path.len()
    }
    pub fn name(&self) -> &str {
        match self.path.last() {
            Some(s) => s.as_ref(),
            None => "",
        }
    }
    pub fn namespace(&self) -> Vec<&str> {
        self.path.iter().map(|s| s.as_ref()).collect()
    }
    pub fn join(&self, joiner: &str) -> String {
        let mut out = String::with_capacity(joiner.len() * self.path.len());
        self.join_write(&mut out, joiner).ok();
        out
    }
    fn join_write(&self, w: &mut impl Write, joiner: &str) -> std::fmt::Result {
        let mut items = self.path.iter();
        match items.next() {
            Some(s) => {
                w.write_str(s)?;
            }
            None => w.write_str("<EMPTY>")?,
        }
        for item in items {
            w.write_str(joiner)?;
            w.write_str(item)?;
        }
        Ok(())
    }
    pub fn push(&mut self, path: &str) {
        self.path.push(Box::from(path))
    }
}

impl Debug for Namepath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.join_write(f, "::")
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
        Ok(Self { path: s.split('.').map(Box::from).collect() })
    }
}
