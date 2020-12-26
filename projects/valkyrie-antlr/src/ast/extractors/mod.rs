use super::*;

impl<'i> Extractor<IdentifierContextAll<'i>> for IdentifierNode {
    fn take(node: Option<Rc<IdentifierContextAll<'i>>>) -> Option<Self> {
        let value = &*node?;
        if let Some(s) = value.UNICODE_ID() {
            return Some(IdentifierNode {
                name: s.get_text(),
                span: Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 },
            });
        }
        if let Some(s) = value.RAW_ID() {
            return Some(IdentifierNode {
                name: s.symbol.text.trim_matches('`').to_string(),
                span: Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 },
            });
        }
        None
    }
}

impl<'i> Extractor<Define_classContextAll<'i>> for ClassDeclaration {
    fn take(node: Option<Rc<Define_classContextAll<'i>>>) -> Option<Self> {
        let raw = &*node?;
        let id = IdentifierNode::take(raw.identifier());
        let span = Range { start: raw.start().start as u32, end: raw.stop().stop as u32 };
        let define = ClassDeclaration {
            kind: ClassKind::Class,
            modifiers: Default::default(),
            identifier: id.unwrap(),
            generic: None,
            base_classes: None,
            auto_traits: vec![],
            body: Default::default(),
            span,
        };
        Some(define)
    }
}

impl<'i> Extractor<Namepath_freeContextAll<'i>> for NamePathNode {
    fn take(node: Option<Rc<Namepath_freeContextAll<'i>>>) -> Option<Self> {
        let raw = node?;
        let span = Range { start: raw.start().start as u32, end: raw.stop().stop as u32 };
        Some(Self { names: IdentifierNode::take_many(raw.identifier_all()), span })
    }
}

impl<'i> Extractor<NamepathContextAll<'i>> for NamePathNode {
    fn take(node: Option<Rc<NamepathContextAll<'i>>>) -> Option<Self> {
        let raw = node?;
        let span = Range { start: raw.start().start as u32, end: raw.stop().stop as u32 };
        Some(Self { names: IdentifierNode::take_many(raw.identifier_all()), span })
    }
}

impl<'i> Extractor<Define_namespaceContextAll<'i>> for NamespaceDeclaration {
    fn take(node: Option<Rc<Define_namespaceContextAll<'i>>>) -> Option<Self> {
        let raw = node?;
        let span = Range { start: raw.start().start as u32, end: raw.stop().stop as u32 };
        Some(Self { kind: NamespaceKind::Shared, path: NamePathNode::take(raw.namepath_free())?.names, span })
    }
}
