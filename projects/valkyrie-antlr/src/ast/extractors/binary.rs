use super::*;

impl<'i> Extractor<Infix_powContextAll<'i>> for OperatorNode {
    fn take_one(node: &Infix_powContextAll<'i>) -> Option<Self> {
        infix_map(&[(node.OP_POW(), ValkyrieOperator::Power), (node.OP_ROOT2(), ValkyrieOperator::Surd)])
    }
}

impl<'i> Extractor<Op_multipleContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_multipleContextAll<'i>) -> Option<Self> {
        infix_map(&[
            (node.OP_MUL(), ValkyrieOperator::Multiply),
            (node.OP_DIV(), ValkyrieOperator::Divide),
            // (o.OP_DIV_DIV(), ValkyrieOperator::DivideFloor),
            (node.OP_DIV_REM(), ValkyrieOperator::DivideRemider), // (o.OP_REM_REM(), ValkyrieOperator::DivideFloor),
        ])
    }
}

impl<'i> Extractor<Op_plusContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_plusContextAll<'i>) -> Option<Self> {
        infix_map(&[(node.OP_ADD(), ValkyrieOperator::Plus), (node.OP_SUB(), ValkyrieOperator::Minus)])
    }
}
impl<'i> Extractor<Op_logicContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_logicContextAll<'i>) -> Option<Self> {
        infix_map(&[
            (node.LOGIC_AND(), ValkyrieOperator::LogicMatrix { mask: LogicMatrix::And }),
            (node.LOGIC_XAND(), ValkyrieOperator::LogicMatrix { mask: LogicMatrix::Xnor }),
            (node.LOGIC_NAND(), ValkyrieOperator::LogicMatrix { mask: LogicMatrix::Nand }),
            (node.LOGIC_OR(), ValkyrieOperator::LogicMatrix { mask: LogicMatrix::Or }),
            (node.LOGIC_XOR(), ValkyrieOperator::LogicMatrix { mask: LogicMatrix::Xor }),
            (node.LOGIC_NOR(), ValkyrieOperator::LogicMatrix { mask: LogicMatrix::Nor }),
        ])
    }
}

impl<'i> Extractor<Op_pipelineContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_pipelineContextAll<'i>) -> Option<Self> {
        infix_map(&[
            (node.OP_LL(), ValkyrieOperator::Placeholder),
            (node.OP_LLL(), ValkyrieOperator::Placeholder),
            (node.OP_LLE(), ValkyrieOperator::Placeholder),
            (node.OP_GG(), ValkyrieOperator::Placeholder),
            (node.OP_GGG(), ValkyrieOperator::Placeholder),
            (node.OP_GGE(), ValkyrieOperator::Placeholder),
        ])
    }
}

impl<'i> Extractor<Op_compareContextAll<'i>> for OperatorNode {
    fn take_one(node: &Op_compareContextAll<'i>) -> Option<Self> {
        infix_map(&[
            (node.OP_NE(), ValkyrieOperator::Equal(false)),
            (node.OP_EQ(), ValkyrieOperator::Equal(true)),
            (node.OP_LT(), ValkyrieOperator::Less { equal: false }),
            (node.OP_LEQ(), ValkyrieOperator::Less { equal: true }),
            (node.OP_GT(), ValkyrieOperator::Greater { equal: false }),
            (node.OP_GEQ(), ValkyrieOperator::Greater { equal: true }),
        ])
    }
}
impl<'i> Extractor<Infix_isContextAll<'i>> for OperatorNode {
    fn take_one(node: &Infix_isContextAll<'i>) -> Option<Self> {
        infix_map(&[
            (node.KW_NOT(), ValkyrieOperator::Is { negative: true }),
            (node.KW_IS(), ValkyrieOperator::Is { negative: false }),
        ])
    }
}
impl<'i> Extractor<Infix_inContextAll<'i>> for OperatorNode {
    fn take_one(node: &Infix_inContextAll<'i>) -> Option<Self> {
        infix_map(&[
            (node.KW_NOT(), ValkyrieOperator::In { negative: true }),
            (node.KW_IN(), ValkyrieOperator::In { negative: false }),
            (node.OP_NOT_IN(), ValkyrieOperator::In { negative: true }),
            (node.OP_IN(), ValkyrieOperator::In { negative: false }),
        ])
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
