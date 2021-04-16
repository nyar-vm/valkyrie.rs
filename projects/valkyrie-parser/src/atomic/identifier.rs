use super::*;
use crate::SlotItemNode;
use nyar_error::NyarError;
use std::num::NonZeroU64;
use valkyrie_ast::{LambdaSlotItem, LambdaSlotNode};

impl crate::NamepathNode {
    pub fn build(&self, ctx: &ProgramContext) -> NamePathNode {
        NamePathNode { names: self.identifier.iter().map(|v| v.build(ctx)).collect() }
    }
}

impl crate::NamepathFreeNode {
    pub fn build(&self, ctx: &ProgramContext) -> NamePathNode {
        NamePathNode { names: self.identifier.iter().map(|v| v.build(ctx)).collect() }
    }
}
impl crate::IdentifierNode {
    pub fn build(&self, ctx: &ProgramContext) -> IdentifierNode {
        match self {
            Self::IdentifierBare(v) => IdentifierNode { name: v.text.to_string(), span: ctx.file.with_range(v.get_range()) },
            Self::IdentifierRaw(v) => {
                IdentifierNode { name: v.identifier_raw_text.text.to_string(), span: ctx.file.with_range(v.get_range()) }
            }
        }
    }
}

impl crate::SlotNode {
    pub fn build(&self, ctx: &ProgramContext) -> Result<LambdaSlotNode, NyarError> {
        Ok(LambdaSlotNode { level: self.op_slot.span.len(), name: build_slot(&self.slot_item, ctx)?, span: self.span.clone() })
    }
}

fn build_slot(node: &Option<SlotItemNode>, ctx: &ProgramContext) -> Result<LambdaSlotItem, NyarError> {
    let node = match node {
        Some(SlotItemNode::Identifier(v)) => return Ok(LambdaSlotItem::Named(v.build(ctx))),
        Some(SlotItemNode::Integer(v)) => v.parse::<u64>(ctx)?,
        None => return Ok(LambdaSlotItem::Nothing),
    };
    match NonZeroU64::new(node) {
        Some(s) => Ok(LambdaSlotItem::Index(s)),
        None => Ok(LambdaSlotItem::MetaType),
    }
}
