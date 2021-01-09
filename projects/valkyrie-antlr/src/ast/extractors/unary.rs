use super::*;

impl<'i> Extractor<ESuffixContext<'i>> for PostfixNode {
    fn take_one(node: &ESuffixContext<'i>) -> Option<Self> {
        let prefix = OperatorNode::take(node.op_suffix())?;
        let base = ExpressionType::take(node.expression())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, base, span })
    }
}
impl<'i> Extractor<EPrefixContext<'i>> for PrefixNode {
    fn take_one(node: &EPrefixContext<'i>) -> Option<Self> {
        let prefix = OperatorNode::take(node.op_prefix())?;
        let base = ExpressionType::take(node.expression())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: prefix, base, span })
    }
}

impl<'i> Extractor<Op_prefixContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_prefixContextAll<'i>) -> Option<Self> {
        let text = node.get_text();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        let kind = match text.as_str() {
            "+" => ValkyrieOperator::Positive,
            "-" => ValkyrieOperator::Negative,
            "&" => ValkyrieOperator::Box,
            "*" => ValkyrieOperator::Unbox,
            "⅟" => ValkyrieOperator::Reciprocal,
            "√" => ValkyrieOperator::Roots(2),
            "∛" => ValkyrieOperator::Roots(3),
            "∜" => ValkyrieOperator::Roots(4),
            _ => unreachable!("Missing prefix {:?}", text),
        };
        Some(Self { kind, span })
    }
}
impl<'i> Extractor<Op_suffixContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_suffixContextAll<'i>) -> Option<Self> {
        let text = node.get_text();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        let kind = match text.as_str() {
            "℃" => ValkyrieOperator::Celsius,
            "℉" => ValkyrieOperator::Fahrenheit,
            "%" => ValkyrieOperator::DivideByDecimalPower(2),
            "‰" => ValkyrieOperator::DivideByDecimalPower(3),
            "‱" => ValkyrieOperator::DivideByDecimalPower(4),
            _ => unreachable!("Missing suffix {:?}", text),
        };
        Some(Self { kind, span })
    }
}
