use super::*;
use antlr_rust::token::Token;
use valkyrie_ast::{CallTermNode, TupleKind, TupleTermNode};

impl<'i> Extractor<Tuple_literalContextAll<'i>> for TupleNode {
    fn take_one(node: &Tuple_literalContextAll<'i>) -> Option<Self> {
        let mut terms = vec![];
        for pair in node.collection_pair_all() {
            match TupleTermNode::take_one(&pair) {
                Some(s) => terms.push(s),
                None => tracing::warn!(""),
            }
        }
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { kind: TupleKind::Tuple, terms, span })
    }
}

impl<'i> Extractor<Collection_pairContextAll<'i>> for TupleTermNode {
    fn take_one(node: &Collection_pairContextAll<'i>) -> Option<Self> {
        let expr = ExpressionType::take(node.expression())?;
        Some(Self { pair: CallTermNode { key: None, value: expr } })
    }
}
