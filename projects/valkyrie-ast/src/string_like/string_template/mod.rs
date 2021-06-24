use super::*;
use crate::StatementNode;

/// `t"template string with {slot + 1:X?}"`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateNode {
    /// The raw string of the number.
    pub items: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `{%=-_ ... _-=%}`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TemplateLineType {
    /// `{% a %}`
    Clear,
}

/// `{% ... %}`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateOpenNode {}

/// `{% expression %}`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateInlineNode {
    /// The inline expression of template
    pub body: Vec<ExpressionKind>,
}

/// `{% end %}`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateCloseNode {
    span: Range<u32>,
}

/// `{# any text as comment #}`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateCommentNode {
    span: Range<u32>,
}
