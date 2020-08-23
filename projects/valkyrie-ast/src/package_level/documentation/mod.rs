use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::ops::Range;
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocumentationNode {
    pub documentation: String,
    /// The range of the node
    pub span: Range<u32>,
}

impl PrettyPrint for DocumentationNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::new();
        for (index, line) in self.documentation.lines().enumerate() {
            if index != 0 {
                terms.push(allocator.hardline())
            }
            terms.push(allocator.text("#? "));
            terms.push(allocator.text(line.to_owned()));
        }
        allocator.concat(terms)
    }
}
