use super::*;

impl<'i> Extractor<EPowContext<'i>> for InfixNode {
    fn take_one(node: &EPowContext<'i>) -> Option<Self> {
        let infix = infix_map(&[(node.OP_POW(), ValkyrieOperator::Power)])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}

impl<'i> Extractor<EMulContext<'i>> for InfixNode {
    fn take_one(node: &EMulContext<'i>) -> Option<Self> {
        let o = node.op_multiple()?;
        let infix = infix_map(&[
            (o.OP_MUL(), ValkyrieOperator::Multiply),
            (o.OP_DIV(), ValkyrieOperator::Divide),
            // (o.OP_DIV_DIV(), ValkyrieOperator::DivideFloor),
            (o.OP_DIV_REM(), ValkyrieOperator::DivideRemider),
            // (o.OP_REM_REM(), ValkyrieOperator::DivideFloor),
        ])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}
impl<'i> Extractor<EPlusContext<'i>> for InfixNode {
    fn take_one(node: &EPlusContext<'i>) -> Option<Self> {
        let o = node.op_plus()?;
        let infix = infix_map(&[(o.OP_ADD(), ValkyrieOperator::Plus), (o.OP_SUB(), ValkyrieOperator::Minus)])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}
impl<'i> Extractor<ELogicContext<'i>> for InfixNode {
    fn take_one(node: &ELogicContext<'i>) -> Option<Self> {
        let o = node.op_logic()?;
        let infix = infix_map(&[
            (o.LOGIC_AND(), ValkyrieOperator::LogicMatrix { mask: 1 }),
            // xand = xnor
            (o.LOGIC_XAND(), ValkyrieOperator::LogicMatrix { mask: 9 }),
            (o.LOGIC_NAND(), ValkyrieOperator::LogicMatrix { mask: 14 }),
            (o.LOGIC_OR(), ValkyrieOperator::LogicMatrix { mask: 7 }),
            // xnand = xor
            (o.LOGIC_XOR(), ValkyrieOperator::LogicMatrix { mask: 6 }),
            (o.LOGIC_NOR(), ValkyrieOperator::LogicMatrix { mask: 8 }),
        ])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}
impl<'i> Extractor<EPipeContext<'i>> for InfixNode {
    fn take_one(node: &EPipeContext<'i>) -> Option<Self> {
        let o = node.op_pipeline()?;
        let infix = infix_map(&[
            (o.OP_LL(), ValkyrieOperator::Placeholder),
            (o.OP_LLL(), ValkyrieOperator::Placeholder),
            (o.OP_LLE(), ValkyrieOperator::Placeholder),
            (o.OP_GG(), ValkyrieOperator::Placeholder),
            (o.OP_GGG(), ValkyrieOperator::Placeholder),
            (o.OP_GGE(), ValkyrieOperator::Placeholder),
        ])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}
impl<'i> Extractor<ECompareContext<'i>> for InfixNode {
    fn take_one(node: &ECompareContext<'i>) -> Option<Self> {
        let o = node.op_compare()?;
        let infix = infix_map(&[
            (o.OP_NE(), ValkyrieOperator::Equal(false)),
            (o.OP_EQ(), ValkyrieOperator::Equal(true)),
            (o.OP_LT(), ValkyrieOperator::Less { equal: false }),
            (o.OP_LEQ(), ValkyrieOperator::Less { equal: true }),
            (o.OP_GT(), ValkyrieOperator::Greater { equal: false }),
            (o.OP_GEQ(), ValkyrieOperator::Greater { equal: true }),
        ])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}
impl<'i> Extractor<EIsAContext<'i>> for InfixNode {
    fn take_one(node: &EIsAContext<'i>) -> Option<Self> {
        let o = node.infix_is()?;
        let infix = infix_map(&[
            (o.KW_NOT(), ValkyrieOperator::Is { negative: true }),
            (o.KW_IS(), ValkyrieOperator::Is { negative: false }),
        ])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}
impl<'i> Extractor<EInContext<'i>> for InfixNode {
    fn take_one(node: &EInContext<'i>) -> Option<Self> {
        let o = node.infix_in()?;
        let infix = infix_map(&[
            (o.KW_NOT(), ValkyrieOperator::In { negative: true }),
            (o.KW_IN(), ValkyrieOperator::In { negative: false }),
            (o.OP_NOT_IN(), ValkyrieOperator::In { negative: true }),
            (o.OP_IN(), ValkyrieOperator::In { negative: false }),
        ])?;
        let lhs = ExpressionType::take(node.lhs.clone())?;
        let rhs = ExpressionType::take(node.rhs.clone())?;
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { operator: infix, lhs, rhs, span })
    }
}
fn infix_map(map: &[(Option<Rc<TerminalNode<ValkyrieAntlrParserContextType>>>, ValkyrieOperator)]) -> Option<OperatorNode> {
    for (source, target) in map {
        if let Some(s) = source {
            let span = Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 };
            return Some(OperatorNode { kind: *target, span });
        }
    }
    return None;
}
