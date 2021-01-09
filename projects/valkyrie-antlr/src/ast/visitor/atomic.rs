use super::*;
use valkyrie_ast::{BooleanNode, NullNode};

impl<'i> Extractor<AtomicContextAll<'i>> for ExpressionType {
    fn take_one(node: &AtomicContextAll<'i>) -> Option<Self> {
        let body: ExpressionType = match node {
            AtomicContextAll::ASpecialContext(s) => {
                let this = s.SPECIAL()?;
                let text = this.get_text();
                let span = Range { start: this.symbol.start as u32, end: this.symbol.stop as u32 };
                match text.as_str() {
                    "true" => BooleanNode { value: true, span }.into(),
                    "false" => BooleanNode { value: false, span }.into(),
                    "null" => NullNode { nil: false, span }.into(),
                    "nil" => NullNode { nil: true, span }.into(),
                    _ => unreachable!("Atom: {}", text),
                }
            }
            AtomicContextAll::ALambdaContext(s) => {
                todo!()
            }
            AtomicContextAll::ANumberContext(s) => NumberLiteralNode::take(s.number_literal())?.into(),
            AtomicContextAll::AStringContext(s) => {
                todo!()
            }
            AtomicContextAll::ANamepathContext(s) => NamePathNode::take(s.namepath())?.into(),
            AtomicContextAll::Error(_) => {
                todo!()
            }
        };
        Some(body)
    }
}

impl<'i> Extractor<Number_literalContextAll<'i>> for NumberLiteralNode {
    fn take_one(node: &Number_literalContextAll<'i>) -> Option<Self> {
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
