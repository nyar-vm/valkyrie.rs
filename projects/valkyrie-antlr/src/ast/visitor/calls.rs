use super::*;

impl<'i> Extractor<Function_callContextAll<'i>> for ApplyCallNode {
    fn take_one(node: &Function_callContextAll<'i>) -> Option<Self> {
        let mut terms = vec![];
        for pair in node.collection_pair_all() {
            match TupleTermNode::take_one(&pair) {
                Some(s) => terms.push(s),
                None => tracing::warn!(""),
            }
        }
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { kind: TupleKind::Tuple, arguments: terms, span })
    }
}
