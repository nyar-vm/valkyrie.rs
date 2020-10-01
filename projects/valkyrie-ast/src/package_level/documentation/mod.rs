use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::ops::Range;
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocumentationNode {
    pub documentation: String,
    /// The range of the node
    pub span: Range<u32>,
}

impl PrettyPrint for DocumentationNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = Vec::new();
        for (index, line) in self.documentation.lines().enumerate() {
            if index != 0 {
                terms.push(theme.hardline())
            }
            terms.push(theme.text("#? "));
            terms.push(theme.text(line.to_owned()));
        }
        theme.concat(terms)
    }
}
