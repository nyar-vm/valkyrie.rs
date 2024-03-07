use super::*;

impl ResolveContext {
    pub fn resolve(&self) -> Result<CanonicalWasi> {
        let mut output = DependentGraph::default();
        {
            for item in self.items.values() {
                match item {
                    ModuleItem::Resource(v) => v.to_lir(self, &mut output)?,
                    ModuleItem::Structure(s) => match &s.external_resource {
                        Some(s) => output += s.clone(),
                        None => {}
                    },
                    ModuleItem::Variant(_) => {}
                    ModuleItem::External(_) => {}
                }
            }
        }
        Ok(CanonicalWasi::new(output)?)
    }
}
