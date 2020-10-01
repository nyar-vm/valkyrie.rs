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
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms.push(theme.keyword("switch"));
        terms.push(theme.space());
        terms.push(theme.text("{"));
        terms.push(theme.hardline());
        let mut inner = Vec::with_capacity(10);
        let len = self.branches.len();
        for (idx, branch) in self.branches.iter().enumerate() {
            inner.push(branch.build(theme));
            if idx == len.saturating_sub(1) {
            }
            else {
                inner.push(theme.hardline());
            }
        }
        terms.push(theme.concat(inner).group().indent(4));
        terms.push(theme.hardline());
        terms.push(theme.text("}"));
        theme.concat(terms)
    }
}
