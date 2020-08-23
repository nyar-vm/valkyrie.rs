#[cfg(feature = "pretty-print")]
mod display;
use super::*;

/// `{ lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaNode {
    pub arguments: Option<LambdaArgumentNode>,
    pub body: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `.caller() { lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaCallNode {
    pub arguments: Option<LambdaArgumentNode>,
    pub body: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `.{ lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaDotNode {
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
        LambdaCallNode { arguments: self.arguments, body: self.body, span: self.span }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn as_lambda_dot(self) -> LambdaDotNode {
        LambdaDotNode { arguments: self.arguments, body: self.body, span: self.span }
    }
}
