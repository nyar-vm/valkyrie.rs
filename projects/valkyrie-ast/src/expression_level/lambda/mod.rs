use super::*;

mod display;

/// `lambda(args) { ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaNode {
    /// The modifiers of the node.
    pub annotations: AnnotationNode,
    /// `method_name<T>()`
    pub generic: Option<ParametersList>,
    /// `method_name(arguments)`
    pub arguments: ArgumentsList,
    /// `: ReturnType / [EffectType]`
    pub returns: FunctionReturnNode,
    /// `{ body }`
    pub body: StatementBlock,
    /// The range of the declaration.
    pub span: Range<u32>,
}

impl ValkyrieNode for LambdaNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

/// `object.{ lambda(args), ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClosureCallNode {
    /// Weather it is a monadic call
    pub monadic: bool,
    /// called
    pub base: ExpressionType,
    /// trailing closure
    pub trailing: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
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
