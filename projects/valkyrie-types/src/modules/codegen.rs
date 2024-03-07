use super::*;
use crate::structures::ValkyrieClassCategory;

impl ResolveContext {
    pub fn resolve(&self) -> Result<CanonicalWasi> {
        let mut output = DependentGraph::default();
        {
            for item in self.items.values() {
                match item {
                    ModuleItem::Structure(s) => s.to_lir(self, &mut output)?,
                    ModuleItem::Variant(_) => {}
                    ModuleItem::External(_) => {}
                }
            }
        }
        Ok(CanonicalWasi::new(output)?)
    }
}
