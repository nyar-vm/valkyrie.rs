use super::*;
use std::ops::AddAssign;
use valkyrie_ast::ParameterSelf;

impl Hir2Mir for FunctionDeclaration {
    type Output = ();
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        let name = ctx.register_item(&self.name);

        let (this, inputs) = self.parameters.split_self();
        match this {
            None => {}
            Some(_) => {}
        }

        for (index, input) in self.parameters.terms.iter().enumerate() {
            match index {
                // extension function
                0 => {
                    println!("Extension functions not supported")
                }
                // normal function
                _ => {
                    println!("{input:?}")
                }
            }
        }
        match ctx.get_foreign_module(&self.annotations, "function", "external", self.keyword) {
            // external function
            Some((wasi_module, wasi_name)) => {
                *ctx += ValkyrieFunction {
                    function_name: name,
                    category: ValkyrieFunctionCategory::External { wasi_module, wasi_name },
                };
                return Ok(());
            }
            None => {}
        }
        return Ok(());
    }
}

impl AddAssign<ValkyrieFunction> for ResolveContext {
    fn add_assign(&mut self, rhs: ValkyrieFunction) {
        self.items.insert(rhs.function_name.clone(), ModuleItem::Function(rhs));
    }
}
