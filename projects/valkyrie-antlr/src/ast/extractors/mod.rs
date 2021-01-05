use super::*;
use valkyrie_ast::{
    ClassFieldDeclaration, ExpressionNode, ExpressionType, FlagsDeclaration, ForLoop, ModifiersNode, OperatorNode,
    PatternExpressionType, PrefixNode, UnionDeclaration, ValkyrieOperator,
};

mod annotations;
mod classes;

impl<'i> Extractor<For_statementContextAll<'i>> for ForLoop {
    fn take_one(node: &For_statementContextAll<'i>) -> Option<Self> {
        // Some(Self {
        //     pattern: PatternExpressionType::Array(),
        //     iterator: Default::default(),
        //     condition: None,
        //     then_body: Default::default(),
        //     else_body: None,
        //     span: Default::default(),
        // })
        todo!()
    }
}

impl<'i> Extractor<Top_expressionContext<'i>> for ExpressionNode {
    fn take_one(node: &Top_expressionContext<'i>) -> Option<Self> {
        let node = ExpressionType::take(node.expression())?;
        Some(Self { type_level: false, body: node, span: Default::default() })
    }
}

impl<'i> Extractor<ExpressionContextAll<'i>> for ExpressionType {
    fn take_one(node: &ExpressionContextAll<'i>) -> Option<Self> {
        let body = match node {
            ExpressionContextAll::EPrefixContext(prefix) => {
                let this = PrefixNode::take_one(prefix)?;
                ExpressionType::Prefix(Box::new(this))
            }
            ExpressionContextAll::ENamepathContext(namepath) => {
                let this = NamePathNode::take(namepath.namepath())?;
                ExpressionType::Symbol(Box::new(this))
            }
            _ => {
                unimplemented!("{:?}", node)
            }
        };
        Some(body)
    }
}

impl<'i> Extractor<EPrefixContext<'i>> for PrefixNode {
    fn take_one(node: &EPrefixContext<'i>) -> Option<Self> {
        let prefix = OperatorNode::take(node.prefix_call())?;
        let base = ExpressionType::take(node.expression())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, base, span })
    }
}

impl<'i> Extractor<Prefix_callContextAll<'i>> for OperatorNode {
    fn take_one(node: &Prefix_callContextAll<'i>) -> Option<Self> {
        let text = node.get_text();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        let kind = match text.as_str() {
            "+" => ValkyrieOperator::Positive,
            _ => {
                unreachable!("Missing prefix {:?}", text)
            }
        };
        Some(Self { kind, span })
    }
}

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
