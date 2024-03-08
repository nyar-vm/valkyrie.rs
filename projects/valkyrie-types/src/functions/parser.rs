use super::*;

impl Hir2Mir for FunctionDeclaration {
    type Output = ();
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        let name = ctx.register_item(&self.name);
        match self.parameters.this {
            // extension function
            Some(_) => {}
            // normal function
            None => {}
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
