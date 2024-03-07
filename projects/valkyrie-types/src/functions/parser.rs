use super::*;

impl Hir2Mir for FunctionDeclaration {
    type Output = ModuleItem;
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        match ctx.get_foreign_module(&self.annotations, "function", "external", self.name.span) {
            Some((wasi_module, wasi_name)) => {
                return Ok(ModuleItem::Function(ValkyrieFunction {
                    function_name: Default::default(),
                    category: ValkyrieFunctionCategory::External { wasi_module, wasi_name },
                }));
            }
            None => {}
        }
        Ok(ModuleItem::Function(todo!()))
    }
}
