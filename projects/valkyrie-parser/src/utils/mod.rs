use crate::{ModifierCallNode, ProgramContext};
use valkyrie_ast::ModifiersNode;

pub fn build_modifiers(m: &[ModifierCallNode], ctx: &ProgramContext) -> ModifiersNode {
    ModifiersNode { terms: m.iter().map(|v| v.identifier.build(ctx)).collect() }
}
