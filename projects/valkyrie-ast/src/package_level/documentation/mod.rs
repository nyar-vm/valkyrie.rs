use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocumentationNode {
    pub documentation: String,
    /// The range of the node
    pub span: Range<u32>,
}
#[cfg(feature = "pretty-print")]
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
