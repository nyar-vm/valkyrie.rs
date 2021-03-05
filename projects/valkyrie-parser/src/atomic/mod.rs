use crate::AtomicNode;
use nyar_error::{Success, Validation};
use valkyrie_ast::{BooleanNode, ExpressionNode, ExpressionType, NumberLiteralNode};

// mod bytes;
// mod identifier;
// mod namepath;
mod number;

impl AtomicNode {
    pub fn build(&self) -> Validation<ExpressionType> {
        let value = match self {
            AtomicNode::Boolean(v) => v.build().into(),
            AtomicNode::Integer(v) => v.build().into(),
            AtomicNode::Namepath(_) => {
                todo!()
            }
            AtomicNode::ProceduralCall(_) => {
                todo!()
            }
            AtomicNode::RangeLiteral(_) => {
                todo!()
            }
            AtomicNode::TupleLiteral(_) => {
                todo!()
            }
        };
        Success { value, diagnostics: vec![] }
    }
}
