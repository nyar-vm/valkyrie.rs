use dashmap::DashMap;

use nyar_error::FileSpan;

use smallvec::SmallVec;
use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    sync::LazyLock,
};

/// A unique identifier used to query the valkyrie object
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieID {
    hash: PathID,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieName {
    hash: NameID,
}

pub struct ValkyrieUniverse {
    paths: DashMap<PathID, ValkyrieIDEntry>,
    names: DashMap<NameID, String>,
}

pub static NAME_UNIVERSE: LazyLock<ValkyrieUniverse> =
    LazyLock::new(|| ValkyrieUniverse { paths: Default::default(), names: Default::default() });

pub static STRING_POOL: LazyLock<ValkyrieUniverse> =
    LazyLock::new(|| ValkyrieUniverse { paths: Default::default(), names: Default::default() });

type NameID = u64;
type PathID = u64;

struct ValkyrieIDEntry {
    name: SmallVec<NameID, 4>,
    span: FileSpan,
}

impl Debug for ValkyrieID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match NAME_UNIVERSE.paths.get(&self.hash) {
            Some(s) => {}
            None => {}
        }

        todo!()
    }
}

impl Display for ValkyrieID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ValkyrieID {
    /// Create a new i
    pub fn new<I>(path: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        todo!()
    }
    /// Set the define location
    pub fn with_location(self, span: FileSpan) -> Self {
        todo!()
    }
    /// Get the names of the file
    pub fn name(&self) -> &str {
        todo!()
    }
    pub fn full_name(&self) -> &[String] {
        todo!()
    }
    pub fn namespace(&self) -> &[String] {
        todo!()
    }
}
