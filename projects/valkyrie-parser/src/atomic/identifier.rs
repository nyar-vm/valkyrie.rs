use super::*;
use crate::SlotItemNode;
use nyar_error::NyarError;
use std::num::NonZeroU64;
use valkyrie_ast::{LambdaSlotItem, LambdaSlotNode};

impl crate::NamepathNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> NamePathNode {
        NamePathNode { names: self.identifier.iter().map(|v| v.build(ctx)).collect() }
    }
}

impl crate::NamepathFreeNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> NamePathNode {
        NamePathNode { names: self.identifier.iter().map(|v| v.build(ctx)).collect() }
    }
}
impl crate::IdentifierNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> IdentifierNode {
        match self {
            Self::IdentifierBare(v) => IdentifierNode { name: v.text.to_string(), span: ctx.file.with_range(v.get_range()) },
            Self::IdentifierRaw(v) => {
                IdentifierNode { name: v.identifier_raw_text.text.to_string(), span: ctx.file.with_range(v.get_range()) }
            }
        }
    }
}

impl crate::SlotNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<LambdaSlotNode> {
        Ok(LambdaSlotNode { level: self.op_slot.span.len(), name: build_slot(&self.slot_item, ctx)?, span: self.span.clone() })
    }
}

fn build_slot(node: &Option<SlotItemNode>, ctx: &mut ProgramState) -> Result<LambdaSlotItem> {
    let node = match node {
        Some(SlotItemNode::Identifier(v)) => return Ok(LambdaSlotItem::Named(v.build(ctx))),
        Some(SlotItemNode::Integer(v)) => v.parse::<u64>(ctx)?,
        None => return Ok(LambdaSlotItem::SelfType),
    };
    match NonZeroU64::new(node) {
        Some(s) => Ok(LambdaSlotItem::Index(s)),
        None => Ok(LambdaSlotItem::MetaType),
    }
}
