use super::*;
use crate::modules::{Hir2Mir, ModuleItem, ResolveContext};

impl Hir2Mir for ClassDeclaration {
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        let symbol = ctx.get_name_path(&self.name);
        let mut class = ValkyrieStructure { symbol, fields: Default::default(), methods: Default::default() };
        for x in self.terms {
            match x {
                ClassTerm::Macro(_) => {}
                ClassTerm::Field(f) => {
                    // let mut field = ValkyrieField::new(&f.name);
                    // field.typing = f.typing;
                    //
                    // class.add_field(field).ok();
                }
                ClassTerm::Method(_) => {}
                ClassTerm::Domain(_) => {}
            }
        }

        match ctx.items.entry(class.symbol.clone()) {
            Entry::Occupied(e) => {
                todo!()
            }
            Entry::Vacant(e) => {
                e.insert(ModuleItem::Structure(class));
            }
        }
        Ok(())
    }
}
