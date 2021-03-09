use super::*;

impl RangeLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<RangeNode> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.subscript_axis {
            match x.build(ctx) {
                Success { value, diagnostics } => {
                    terms.push(value);
                    errors.extend(diagnostics);
                }
                Failure { fatal, diagnostics } => {
                    errors.push(fatal);
                    errors.extend(diagnostics);
                }
            }
        }
        Success { value: RangeNode { kind: RangeKind::Ordinal, terms, span: self.span.clone() }, diagnostics: errors }
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
        self.index.build(ctx).map(|v| RangeTermNode::Index { index: v.body })
    }
}

impl SubscriptRangeNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<RangeTermNode> {
        let head = match &self.head {
            Some(s) => Some(s.build(ctx)?.body),
            None => None,
        };
        let tail = match &self.tail {
            Some(s) => Some(s.build(ctx)?.body),
            None => None,
        };
        let step = match &self.step {
            Some(s) => Some(s.build(ctx)?.body),
            None => None,
        };
        Success { value: RangeTermNode::Range { head, tail, step }, diagnostics: vec![] }
    }
}
