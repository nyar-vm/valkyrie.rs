use super::*;

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
        Ok(LambdaSlotNode { level: self.op_slot.span.len(), item: self.item(ctx)?, span: self.span.clone() })
    }
    fn item(&self, ctx: &mut ProgramState) -> Result<LambdaSlotItem> {
        match &self.slot_item {
            Some(s) => s.build(ctx),
            None => return Ok(LambdaSlotItem::SelfType),
        }
    }
}

impl crate::SlotItemNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<LambdaSlotItem> {
        let value = match self {
            Self::Identifier(v) => LambdaSlotItem::Named(v.build(ctx)),
            Self::Integer(v) => match NonZeroU64::new(v.parse::<u64>(ctx)?) {
                Some(s) => LambdaSlotItem::Index(s),
                None => LambdaSlotItem::MetaType,
            },
        };
        Ok(value)
    }
}
