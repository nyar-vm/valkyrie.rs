use alloc::{borrow::ToOwned, string::String};
use core::ops::Range;
use pretty_print::{helpers::PrettySequence, PrettyPrint, PrettyProvider, PrettyTree};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocumentationNode {
    pub documentation: String,
    /// The range of the node
    pub span: Range<u32>,
}

impl PrettyPrint for DocumentationNode {
    fn pretty(&self, _: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(0);
        for (index, line) in self.documentation.lines().enumerate() {
            if index != 0 {
                terms += PrettyTree::Hardline
            }
            terms += "#? ";
            terms += line.to_owned();
        }
        terms.into()
    }
}
