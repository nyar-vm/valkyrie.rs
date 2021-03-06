use super::*;
use valkyrie_ast::TupleKeyType;

impl RangeLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleKeyType> {
        let value = match self {
            AtomicNode::Special(v) => v.build().into(),
            AtomicNode::Integer(v) => v.build().into(),
            AtomicNode::Namepath(v) => v.build(ctx).into(),
            AtomicNode::ProceduralCall(_) => {
                todo!()
            }
            AtomicNode::RangeLiteral(v) => {
                todo!()
            }
            AtomicNode::TupleLiteral(_) => {
                todo!()
            }
        };
        Success { value, diagnostics: vec![] }
    }
}
