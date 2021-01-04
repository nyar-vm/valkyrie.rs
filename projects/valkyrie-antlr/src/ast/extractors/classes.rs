use super::*;
use valkyrie_ast::ClassMethodDeclaration;

impl<'i> Extractor<Define_classContextAll<'i>> for ClassDeclaration {
    fn take_one(node: &Define_classContextAll<'i>) -> Option<Self> {
        let id = IdentifierNode::take(node.identifier())?;
        let modifiers = ModifiersNode::take(node.modifiers()).unwrap_or_default();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        // valid class must have block;
        let blocks = node.class_block()?;
        let fields = ClassFieldDeclaration::take_many(&blocks.class_field_all());
        let methods = ClassMethodDeclaration::take_many(&blocks.class_method_all());
        Some(ClassDeclaration {
            kind: ClassKind::Class,
            name: id,
            modifiers,
            generic: None,
            base_classes: None,
            auto_traits: vec![],
            fields,
            methods,
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

impl<'i> Extractor<Class_methodContextAll<'i>> for ClassMethodDeclaration {
    fn take_one(node: &Class_methodContextAll<'i>) -> Option<Self> {
        let id = IdentifierNode::take(node.modified_namepath())?;
        let modifiers = ModifiersNode::take(node.modified_namepath()).unwrap_or_default();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self {
            document: Default::default(),
            modifiers,
            method_name: id,
            generic: None,
            arguments: Default::default(),
            return_type: None,
            effect_type: None,
            body: None,
        })
    }
}
