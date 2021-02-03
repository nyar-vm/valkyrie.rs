use super::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ValkyrieDict {
    pub raw: HashMap<ValkyrieValue, ValkyrieValue>,
}

impl Debug for ValkyrieDict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().finish()
    }
}

unsafe impl GcSafe for ValkyrieDict {}
unsafe impl GcDrop for ValkyrieDict {}
unsafe impl Scan for ValkyrieDict {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        for (key, value) in self.raw.iter() {
            scanner.scan(key);
            scanner.scan(value);
        }
    }
}
impl Default for ValkyrieDict {
    fn default() -> Self {
        Self { raw: Default::default() }
    }
}
impl ValkyrieDict {
    pub fn clear(&mut self) {
        self.raw.clear();
    }
}

impl Serialize for ValkyrieDict {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(self.raw.len()))?;
        for (key, value) in self.raw.iter() {
            seq.serialize_entry(key, value)?;
        }
        seq.end()
    }
}
