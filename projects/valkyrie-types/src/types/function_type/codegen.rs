use super::*;
use nyar_wasm::FunctionType;

impl FromFrontend<FunctionDefinition> for FunctionDeclaration {
    fn build(&self, state: &mut ValkyrieCodegen) -> Result<FunctionDefinition> {
        let mut output = FunctionDefinition::new(&state.current_namespace, &self.name)?;

        Ok(output)
    }
}

impl IntoBackend<FunctionType> for FunctionDefinition {
    fn build(&self, state: &mut ValkyrieCodegen) -> Result<FunctionType> {
        let mut output = FunctionType::new(Symbol::from(self.name()));

        Ok(output)
    }
}
