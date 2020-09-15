use super::*;

/// `switch { when a > 0: a, else: 0}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwitchStatement {
    pub branches: Vec<PatternBranch>,
    /// The range of the node
    pub span: Range<u32>,
}

impl PrettyPrint for SwitchStatement {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.join(&self.branches, ",")
    }
}
