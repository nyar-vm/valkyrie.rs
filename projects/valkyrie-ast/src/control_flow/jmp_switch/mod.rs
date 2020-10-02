use super::*;
use pretty_print::PrettyBuilder;

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
        terms += theme.keyword("switch");
        terms += " ";
        terms += "{";
        terms += PrettyTree::Hardline;
        let mut inner = PrettySequence::new(10);
        let len = self.branches.len();
        for (idx, branch) in self.branches.iter().enumerate() {
            inner += branch.pretty(theme);
            if idx == len.saturating_sub(1) {
            }
            else {
                inner += PrettyTree::Hardline;
            }
        }
        terms += inner.indent(4);
        terms += PrettyTree::Hardline;
        terms += "}";
        terms.into()
    }
}
