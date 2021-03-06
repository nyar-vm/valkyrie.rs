use super::*;
use valkyrie_ast::{IdentifierNode, NamePathNode};
use yggdrasil_rt::YggdrasilNode;

impl crate::NamepathNode {
    pub fn build(&self, ctx: &ProgramContext) -> NamePathNode {
        NamePathNode { names: self.identifier.iter().map(|v| v.build(ctx)).collect() }
    }
}

impl crate::IdentifierNode {
    pub fn build(&self, ctx: &ProgramContext) -> IdentifierNode {
        match self {
            Self::IdentifierBare(v) => {
                IdentifierNode { name: v.text.to_string(), span: ctx.file.with_range(v.get_range().unwrap_or_default()) }
            }
            Self::IdentifierRaw(v) => IdentifierNode {
                name: v.identifier_raw_text.text.to_string(),
                span: ctx.file.with_range(v.get_range().unwrap_or_default()),
            },
        }
    }
}
