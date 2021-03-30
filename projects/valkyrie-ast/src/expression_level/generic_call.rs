use super::*;

/// `A⦓T⦔` or `A::<T>`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCallNode {
    /// `this?::<T>`
    pub monadic: bool,
    /// `A`
    pub base: ExpressionType,
    /// The raw string of the number.
    pub terms: Vec<TupleTermNode>,
    /// The associated type or method
    /// `A::<T as Trait>::Type::method`
    pub associated: Vec<IdentifierNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl ValkyrieNode for GenericCallNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

impl GenericCallNode {
    /// Replace placeholder with actual expression
    pub fn with_base(self, base: ExpressionType) -> Self {
        Self { base, ..self }
    }
}
