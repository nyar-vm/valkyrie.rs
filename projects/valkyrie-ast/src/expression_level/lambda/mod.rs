use super::*;

mod display;

/// `{ lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaNode {
    pub arguments: Option<FunctionBlock>,
    pub body: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `object.caller() { lambda(args), ... }`
/// `object.{ lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClosureCallNode {
    /// called
    pub base: ExpressionType,
    /// Weather it is a monadic call
    pub monadic: bool,
    ///
    pub caller: ClosureCaller,
    /// trailing closure
    pub trailing: Option<FunctionBlock>,
    /// The range of the node
    pub span: Range<u32>,
}

/// The key of tuple
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClosureCaller {
    /// `object() { }`
    Normal,
    /// `object.{ $1 + 1 }`
    Lambda,
    /// `object.1() { }`
    Integer(NonZeroU64),
    /// `object.method() { }`, maybe macro call
    Internal(IdentifierNode),
}

/// `lambda(args)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionBlock {
    /// The raw string of the number.
    pub terms: Vec<StatementNode>,
    /// The range of the number.
    pub range: Range<u32>,
}
impl ValkyrieNode for ClosureCallNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl ClosureCallNode {
    /// Replace placeholder with actual expression
    pub fn with_base(self, base: ExpressionType) -> Self {
        Self { base, ..self }
    }
}
