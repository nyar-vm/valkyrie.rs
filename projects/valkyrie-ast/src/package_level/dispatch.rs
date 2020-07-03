use super::*;

impl Display for TopStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TopStatementNode::Namespace(v) => Display::fmt(v, f),
            TopStatementNode::Class(v) => Display::fmt(v, f),
            TopStatementNode::Expression(v) => Display::fmt(v, f),
            TopStatementNode::Import(v) => Display::fmt(v, f),
        }
    }
}

impl From<ImportStatementNode> for TopStatementNode {
    fn from(value: ImportStatementNode) -> Self {
        TopStatementNode::Import(Box::new(value))
    }
}

impl Display for ReplStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ReplStatementNode::DeclareClass(v) => Display::fmt(v, f),
            ReplStatementNode::Expression(v) => Display::fmt(v, f),
            ReplStatementNode::Loop(v) => Display::fmt(v, f),
        }
    }
}

impl Display for FunctionStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            FunctionStatementNode::Expression(v) => Display::fmt(v, f),
            FunctionStatementNode::Loop(v) => Display::fmt(v, f),
        }
    }
}

impl From<NamespaceDeclarationNode> for TopStatementNode {
    fn from(value: NamespaceDeclarationNode) -> Self {
        TopStatementNode::Namespace(Box::new(value))
    }
}

impl From<ExpressionNode> for TopStatementNode {
    fn from(value: ExpressionNode) -> Self {
        TopStatementNode::Expression(Box::new(value))
    }
}

impl From<ExpressionNode> for ReplStatementNode {
    fn from(value: ExpressionNode) -> Self {
        ReplStatementNode::Expression(Box::new(value))
    }
}

impl From<ExpressionNode> for FunctionStatementNode {
    fn from(value: ExpressionNode) -> Self {
        FunctionStatementNode::Expression(Box::new(value))
    }
}
