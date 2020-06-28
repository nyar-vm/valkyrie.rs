use super::*;

impl Display for TopStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TopStatementNode::DeclareNamespace(v) => Display::fmt(v, f),
            TopStatementNode::DeclareClass(v) => Display::fmt(v, f),
            TopStatementNode::Expression(v) => Display::fmt(v, f),
        }
    }
}

impl Display for ReplStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ReplStatementNode::DeclareClass(v) => Display::fmt(v, f),
            ReplStatementNode::Expression(v) => Display::fmt(v, f),
        }
    }
}

impl Display for FunctionStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            FunctionStatementNode::Expression(v) => Display::fmt(v, f),
        }
    }
}

impl From<NamespaceDeclarationNode> for TopStatementNode {
    fn from(value: NamespaceDeclarationNode) -> Self {
        TopStatementNode::DeclareNamespace(Box::new(value))
    }
}

impl From<TermExpressionNode> for TopStatementNode {
    fn from(value: TermExpressionNode) -> Self {
        TopStatementNode::Expression(Box::new(value))
    }
}

impl From<TermExpressionNode> for ReplStatementNode {
    fn from(value: TermExpressionNode) -> Self {
        ReplStatementNode::Expression(Box::new(value))
    }
}
impl From<TermExpressionNode> for FunctionStatementNode {
    fn from(value: TermExpressionNode) -> Self {
        FunctionStatementNode::Expression(Box::new(value))
    }
}
