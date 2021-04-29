use super::*;
use valkyrie_ast::SubscriptCallNode;

impl RangeLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<RangeNode> {
        let mut errors = vec![];
        let mut value = RangeNode { kind: RangeKind::Ordinal, terms: vec![], span: Default::default() };
        match self {
            Self::RangeLiteralIndex0(v) => {
                for x in &v.subscript_axis {
                    x.build(ctx).append(&mut value.terms, &mut errors)
                }
                value.span = v.span.clone()
            }
            Self::RangeLiteralIndex1(v) => {
                value.kind = RangeKind::Offset;
                for x in &v.subscript_axis {
                    x.build(ctx).append(&mut value.terms, &mut errors)
                }
                value.span = v.span.clone()
            }
        }
        Success { value, diagnostics: errors }
    }
}

impl SubscriptAxisNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<RangeTermNode> {
        match self {
            SubscriptAxisNode::SubscriptOnly(v) => v.build(ctx),
            SubscriptAxisNode::SubscriptRange(v) => v.build(ctx),
        }
    }
}

impl SubscriptOnlyNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<RangeTermNode> {
        self.index.build(ctx).map(|v| RangeTermNode::Index { index: v })
    }
}

impl SubscriptRangeNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<RangeTermNode> {
        let head = match &self.head {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        let tail = match &self.tail {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        let step = match &self.step {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Success { value: RangeTermNode::Range { head, tail, step }, diagnostics: vec![] }
    }
}
impl crate::RangeCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<SubscriptCallNode> {
        let monadic = self.op_and_then.is_some();
        let terms = self.range_literal.build(ctx)?.terms;
        Success {
            value: SubscriptCallNode {
                kind: RangeKind::Ordinal,
                base: ExpressionType::Placeholder,
                monadic,
                terms,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
