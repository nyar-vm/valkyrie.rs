use super::*;
use crate::ExpressionNode;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringFormatter {
    /// The raw string of the number.
    pub items: Vec<ExpressionNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `expr.format(r"?x")`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionFormatted {
    /// The raw string of the number.
    pub body: ExpressionNode,
    /// The format arguments
    pub arguments: String,
    /// The range of the node
    pub span: Range<u32>,
}
impl ValkyrieNode for ExpressionFormatted {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl PrettyPrint for ExpressionFormatted {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        todo!()
    }
}
