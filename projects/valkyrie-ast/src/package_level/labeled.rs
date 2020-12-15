use super::*;

/// `⁜ label.context.1`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GotoStatement {
    pub path: Vec<IdentifierNode>,
    pub span: Range<u32>,
}

/// `※label.context.1`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LabelStatement {
    /// The label path
    pub path: Vec<IdentifierNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl PrettyPrint for LabelStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        todo!()
    }
}

impl PrettyPrint for GotoStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        todo!()
    }
}
