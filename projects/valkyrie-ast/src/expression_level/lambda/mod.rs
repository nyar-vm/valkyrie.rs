use super::*;

mod display;

/// `{ lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaNode {
    pub arguments: Option<LambdaArgumentNode>,
    pub body: Vec<StatementNode>,
    pub span: Range<u32>,
}

/// `.caller() { lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaCallNode {
    pub base: ExpressionNode,
    pub arguments: Option<LambdaArgumentNode>,
    pub body: Vec<StatementNode>,
    pub span: Range<u32>,
}

/// `.{ lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaDotNode {
    pub base: ExpressionNode,
    pub arguments: Option<LambdaArgumentNode>,
    pub body: Vec<StatementNode>,
    pub span: Range<u32>,
}

/// `lambda(args)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<StatementNode>,
    /// The range of the number.
    pub range: Range<u32>,
}

impl LambdaNode {
    #[allow(clippy::wrong_self_convention)]
    pub fn as_lambda_call(self) -> LambdaCallNode {
        LambdaCallNode { base: ExpressionNode::default(), arguments: self.arguments, body: self.body, span: self.span }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn as_lambda_dot(self) -> LambdaDotNode {
        LambdaDotNode { base: ExpressionNode::default(), arguments: self.arguments, body: self.body, span: self.span }
    }
}

impl LambdaCallNode {
    pub fn rebase(mut self: Box<Self>, base: ExpressionBody) -> Box<Self> {
        self.base.body = base;
        self
    }
}

impl LambdaDotNode {
    pub fn rebase(mut self: Box<Self>, base: ExpressionBody) -> Box<Self> {
        self.base.body = base;
        self
    }
}
