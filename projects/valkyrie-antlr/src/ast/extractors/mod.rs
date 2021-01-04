use super::*;
use valkyrie_ast::{ClassFieldDeclaration, FlagsDeclaration, ModifiersNode, UnionDeclaration};

mod annotations;
mod classes;

impl<'i> Extractor<IdentifierContextAll<'i>> for IdentifierNode {
    fn take_one(node: &IdentifierContextAll<'i>) -> Option<Self> {
        if let Some(s) = node.UNICODE_ID() {
            return Some(IdentifierNode {
                name: s.get_text(),
                span: Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 },
            });
        }
        if let Some(s) = node.RAW_ID() {
            return Some(IdentifierNode {
                name: s.symbol.text.trim_matches('`').to_string(),
                span: Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 },
            });
        }
        None
    }
}

impl<'i> Extractor<Define_unionContextAll<'i>> for UnionDeclaration {
    fn take_one(node: &Define_unionContextAll<'i>) -> Option<Self> {
        let id = IdentifierNode::take(node.identifier())?;
        let modifiers = ModifiersNode::take(node.modifiers()).unwrap_or_default();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self {
            document: Default::default(),
            name: id,
            modifiers,
            layout: None,
            derive_traits: vec![],
            body: Default::default(),
            span,
        })
    }
}

impl<'i> Extractor<Define_bitflagsContextAll<'i>> for FlagsDeclaration {
    fn take_one(node: &Define_bitflagsContextAll<'i>) -> Option<Self> {
        let id = IdentifierNode::take(node.identifier())?;
        let modifiers = ModifiersNode::take(node.modifiers()).unwrap_or_default();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self {
            documentation: Default::default(),
            name: id,
            modifiers,
            layout: None,
            implements: vec![],
            body: Default::default(),
            span,
        })
    }
}

impl<'i> Extractor<Namepath_freeContextAll<'i>> for NamePathNode {
    fn take_one(node: &Namepath_freeContextAll<'i>) -> Option<Self> {
        Some(Self { names: IdentifierNode::take_many(&node.identifier_all()) })
    }
}

impl<'i> Extractor<NamepathContextAll<'i>> for NamePathNode {
    fn take_one(node: &NamepathContextAll<'i>) -> Option<Self> {
        Some(Self { names: IdentifierNode::take_many(&node.identifier_all()) })
    }
}

impl<'i> Extractor<Define_namespaceContextAll<'i>> for NamespaceDeclaration {
    fn take_one(node: &Define_namespaceContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { kind: NamespaceKind::Shared, path: NamePathNode::take(node.namepath_free())?.names, span })
    }
}
