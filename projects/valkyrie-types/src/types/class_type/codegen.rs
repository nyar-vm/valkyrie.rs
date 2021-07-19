use super::*;

impl FromFrontend<ValkyrieStructure> for ClassDeclaration {
    fn build(&self, state: &mut ValkyrieCodegen) -> Result<ValkyrieStructure> {
        let mut output = ValkyrieStructure::new(&state.current_namespace, &self.name);
        // state.register_class(output.clone());

        for x in &self.terms {
            match x {
                ClassTerm::Macro(_) => {}
                ClassTerm::Field(v) => match output.add_field(v.build(state)?) {
                    Err(e) if !state.interactive => state.add_error(e),
                    _ => {}
                },
                ClassTerm::Method(v) => match output.add_method(v.build(state)?) {
                    Err(e) if !state.interactive => state.add_error(e),
                    _ => {}
                },
                ClassTerm::Domain(_) => {}
            }
        }
        Ok(output)
    }
}
