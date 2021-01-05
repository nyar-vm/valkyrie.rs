use super::*;

impl<'i> Extractor<EPlusContext<'i>> for InfixNode {
    fn take_one(node: &EPlusContext<'i>) -> Option<Self> {
        let prefix = OperatorNode::take(node.op_plus())?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, lhs, rhs, span })
    }
}

impl<'i> Extractor<Op_plusContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_plusContextAll<'i>) -> Option<Self> {
        let text = node.get_text();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        let kind = match text.as_str() {
            "+" => ValkyrieOperator::Plus,
            "-" => ValkyrieOperator::Minus,
            _ => unreachable!("Missing infix {:?}", text),
        };
        Some(Self { kind, span })
    }
}
