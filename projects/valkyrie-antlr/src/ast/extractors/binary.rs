use super::*;
use antlr_rust::tree::Tree;
impl<'i> Extractor<EPowContext<'i>> for InfixNode {
    fn take_one(node: &EPowContext<'i>) -> Option<Self> {
        let prefix = OperatorNode::take(node.OP_POW())?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, lhs, rhs, span })
    }
}
impl<'i> Extractor<EMulContext<'i>> for InfixNode {
    fn take_one(node: &EMulContext<'i>) -> Option<Self> {
        let prefix = OperatorNode::take_one_of(vec![node.OP_MUL(), node.OP_DIV()])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, lhs, rhs, span })
    }
}
impl<'i> Extractor<EPlusContext<'i>> for InfixNode {
    fn take_one(node: &EPlusContext<'i>) -> Option<Self> {
        let prefix = OperatorNode::take_one_of(vec![node.OP_ADD(), node.OP_SUB()])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, lhs, rhs, span })
    }
}

impl<'i> Extractor<ECompareContext<'i>> for InfixNode {
    fn take_one(node: &ECompareContext<'i>) -> Option<Self> {
        let prefix = infix_map(&[
            (node.OP_EQ(), ValkyrieOperator::Equal(true)),
            (node.OP_NE(), ValkyrieOperator::Equal(false)),
            (node.OP_LT(), ValkyrieOperator::Less),
            (node.OP_GT(), ValkyrieOperator::Equal(false)),
            (node.OP_LEQ(), ValkyrieOperator::Equal(false)),
            (node.OP_GEQ(), ValkyrieOperator::Equal(false)),
        ])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, lhs, rhs, span })
    }
}

fn infix_map(map: &[(Option<Rc<TerminalNode<ValkyrieAntlrParserContextType>>>, ValkyrieOperator)]) -> Option<OperatorNode> {
    for (source, target) in map {
        match source {
            Some(s) => {
                let span = Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 };
                return Some(OperatorNode { kind: *target, span });
            }
            None => {}
        }
    }
    return None;
}

impl<'i> Extractor<TerminalNode<'i, ValkyrieAntlrParserContextType>> for OperatorNode {
    fn take_one(node: &TerminalNode<'i, ValkyrieAntlrParserContextType>) -> Option<Self> {
        let text = node.get_text();
        let span = Range { start: node.symbol.start as u32, end: node.symbol.stop as u32 };
        let kind = match text.as_str() {
            "+" => ValkyrieOperator::Plus,
            "-" => ValkyrieOperator::Minus,
            "^" => ValkyrieOperator::Power,
            "*" => ValkyrieOperator::Multiply,
            "/" => ValkyrieOperator::Divide,
            "/%" => ValkyrieOperator::DivideRemider,
            "//" => ValkyrieOperator::DivideFloor,
            ">" => ValkyrieOperator::Greater,
            "<" => ValkyrieOperator::Less,
            _ => unreachable!("Missing infix {:?}", text),
        };
        Some(Self { kind, span })
    }
}
