use super::*;
use valkyrie_ast::ApplyCallItem;

impl<'i> Extractor<Function_callContextAll<'i>> for ApplyCallNode {
    fn take_one(node: &Function_callContextAll<'i>) -> Option<Self> {
        let monadic = node.OP_AND_THEN().is_some();
        let mut terms = vec![];
        for pair in node.tuple_call_body() {
            match TupleTermNode::take_one(&pair) {
                Some(s) => terms.push(s),
                None => tracing::warn!(""),
            }
        }
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { base: Default::default(), monadic, caller: Default::default(), arguments: None, span })
    }
}

impl<'i> Extractor<Tuple_call_itemContextAll<'i>> for ApplyCallItem {
    fn take_one(node: &Tuple_call_itemContextAll<'i>) -> Option<Self> {
        let modifiers = ModifiersNode::take_many(&node.mods)?;

        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { modifiers, term: CallTermNode { key: None, value: () } })
    }
}
