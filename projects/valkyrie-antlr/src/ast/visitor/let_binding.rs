use super::*;

impl<'i> Extractor<Let_bindingContextAll<'i>> for LetBindNode {
    fn take_one(node: &Let_bindingContextAll<'i>) -> Option<Self> {
        let pat = LetPattern::take(node.let_pattern())?;
        let typ = ExpressionType::take(node.type_hint());
        let rhs = ExpressionNode::take(node.expression_root());
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self {
            pattern: pat,
            type_hint: typ,
            body: rhs,
            span,
        })
    }
}
impl<'i> Extractor<Let_patternContextAll<'i>> for LetPattern {
    fn take_one(node: &Let_patternContextAll<'i>) -> Option<Self> {

        let mut terms = vec![];
        let pat = LetPattern::take
        let rhs = ExpressionNode::take(node.expression_root());

        for pair in node.collection_pair_all() {
            match TupleTermNode::take_one(&pair) {
                Some(s) => terms.push(s),
                None => tracing::warn!(""),
            }
        }
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self {
            pattern: (),
            type_hint: None,
            body: rhs,
        })
    }
}
