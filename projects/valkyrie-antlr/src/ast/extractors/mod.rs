use super::*;
use valkyrie_ast::{ClassFieldDeclaration, FlagsDeclaration, ModifiersNode, UnionDeclaration};

impl<'i> Extractor<ModifiersContext<'i>> for ModifiersNode {
    fn take_one(node: &ModifiersContextAll<'i>) -> Option<Self> {
        Some(Self { terms: IdentifierNode::take_many(&node.identifier_all()) })
    }
}

impl<'i> Extractor<Modified_identifierContextAll<'i>> for ModifiersNode {
    fn take_one(node: &Modified_identifierContextAll<'i>) -> Option<Self> {
        // let span = Range { start: raw.start().start as u32, end: raw.stop().stop as u32 };
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
impl<'i> Extractor<Define_classContextAll<'i>> for ClassDeclaration {
    fn take_one(node: &Define_classContextAll<'i>) -> Option<Self> {
        let id = IdentifierNode::take(node.identifier())?;
        let modifiers = ModifiersNode::take(node.modifiers()).unwrap_or_default();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(ClassDeclaration {
            kind: ClassKind::Class,
            name: id,
            modifiers,
            generic: None,
            base_classes: None,
            auto_traits: vec![],
            body: Default::default(),
            span,
        })
    }
}
impl<'i> Extractor<Class_fieldContextAll<'i>> for ClassFieldDeclaration {
    fn take_one(node: &Class_fieldContextAll<'i>) -> Option<Self> {
        let id = IdentifierNode::take(node.modified_identifier())?;
        let modifiers = ModifiersNode::take(node.modified_identifier()).unwrap_or_default();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { document: Default::default(), modifiers, field_name: id, r#type: None, default: None, span })
    }
}

impl<'i> Extractor<Namepath_freeContextAll<'i>> for NamePathNode {
    fn take_one(node: &Namepath_freeContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { names: IdentifierNode::take_many(&node.identifier_all()), span })
    }
}

impl<'i> Extractor<NamepathContextAll<'i>> for NamePathNode {
    fn take_one(node: &NamepathContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { names: IdentifierNode::take_many(&node.identifier_all()), span })
    }
}

impl<'i> Extractor<Define_namespaceContextAll<'i>> for NamespaceDeclaration {
    fn take_one(node: &Define_namespaceContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { kind: NamespaceKind::Shared, path: NamePathNode::take(node.namepath_free())?.names, span })
    }
}
