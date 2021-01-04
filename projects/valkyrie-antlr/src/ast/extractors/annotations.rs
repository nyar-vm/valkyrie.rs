use super::*;

impl<'i> Extractor<ModifiersContext<'i>> for ModifiersNode {
    fn take_one(node: &ModifiersContextAll<'i>) -> Option<Self> {
        Some(Self { terms: IdentifierNode::take_many(&node.identifier_all()) })
    }
}

impl<'i> Extractor<Modified_identifierContextAll<'i>> for ModifiersNode {
    fn take_one(node: &Modified_identifierContextAll<'i>) -> Option<Self> {
        Some(Self { terms: IdentifierNode::take_many(&node.mods) })
    }
}
impl<'i> Extractor<Modified_identifierContextAll<'i>> for IdentifierNode {
    fn take_one(node: &Modified_identifierContextAll<'i>) -> Option<Self> {
        let id = node.id.clone()?;
        IdentifierNode::take_one(&*id)
    }
}

// impl<'i> Extractor<ModifiersContextAll<'i>> for ModifiersNode {
//     fn take(node: Option<Rc<ModifiersContextAll<'i>>>) -> Option<Self> {
//         let raw = node?;
//         // let span = Range { start: raw.start().start as u32, end: raw.stop().stop as u32 };
//         Some(Self { terms: IdentifierNode::take_many(raw.identifier_all()) })
//     }
// }
