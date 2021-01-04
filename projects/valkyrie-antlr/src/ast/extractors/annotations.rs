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

impl<'i> Extractor<Modified_namepathContextAll<'i>> for ModifiersNode {
    fn take_one(node: &Modified_namepathContextAll<'i>) -> Option<Self> {
        Some(Self { terms: IdentifierNode::take_many(&node.mods) })
    }
}

impl<'i> Extractor<Modified_namepathContextAll<'i>> for NamePathNode {
    fn take_one(node: &Modified_namepathContextAll<'i>) -> Option<Self> {
        Some(Self { names: IdentifierNode::take_many(&node.path) })
    }
}
