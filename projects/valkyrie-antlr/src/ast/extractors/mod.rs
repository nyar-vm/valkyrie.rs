use super::*;
use valkyrie_ast::{
    ClassFieldDeclaration, ExpressionNode, ExpressionType, FlagsDeclaration, ForLoop, InfixNode, ModifiersNode,
    NumberLiteralNode, OperatorNode, PatternExpressionType, PostfixNode, PrefixNode, UnionDeclaration, ValkyrieOperator,
};

mod annotations;
mod binary;
mod classes;
mod unary;

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
            ExpressionContextAll::EPrefixContext(prefix) => PrefixNode::take_one(prefix)?.into(),
            ExpressionContextAll::ESuffixContext(suffix) => PostfixNode::take_one(suffix)?.into(),
            ExpressionContextAll::EPowContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EPlusContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EMulContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EIsAContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::ECompareContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EPipeContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EInContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::ELogicContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::ENamepathContext(namepath) => NamePathNode::take(namepath.namepath())?.into(),
            ExpressionContextAll::EGroupContext(group) => ExpressionType::take(group.expression())?,
            ExpressionContextAll::ENumberContext(plus) => NumberLiteralNode::take_one(plus)?.into(),
            _ => {
                unimplemented!("{:?}", node)
            }
        };
        Some(body)
    }
}
impl<'i> Extractor<Type_expressionContextAll<'i>> for ExpressionType {
    fn take_one(node: &Type_expressionContextAll<'i>) -> Option<Self> {
        let body = match node {
            Type_expressionContextAll::TNamepathContext(namepath) => NamePathNode::take(namepath.namepath())?.into(),
            _ => {
                unimplemented!("{:?}", node)
            }
        };
        Some(body)
    }
}
impl<'i> Extractor<ENumberContext<'i>> for NumberLiteralNode {
    fn take_one(node: &ENumberContext<'i>) -> Option<Self> {
        let node = node.number_literal()?;
        let value = node.number()?.get_text();
        let suffix = IdentifierNode::take(node.identifier());
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { value, unit: suffix, span })
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
