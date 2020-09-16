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
        let mut terms = Vec::with_capacity(10);
        terms.push(allocator.keyword("switch"));
        terms.push(allocator.space());
        terms.push(allocator.text("{"));
        terms.push(allocator.hardline());
        let mut inner = Vec::with_capacity(10);
        let len = self.branches.len();
        for (idx, branch) in self.branches.iter().enumerate() {
            inner.push(branch.build(allocator));
            if idx == len.saturating_sub(1) {
            }
            else {
                inner.push(allocator.hardline());
            }
        }
        terms.push(allocator.concat(inner).group().indent(4));
        terms.push(allocator.hardline());
        terms.push(allocator.text("}"));
        allocator.concat(terms)
    }
}
