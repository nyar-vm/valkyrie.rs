use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocumentationList {
    /// Spans of this documentation
    pub terms: Vec<StringTextNode>,
}

impl DocumentationList {
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for DocumentationList {
    fn pretty(&self, _: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(0);
        for (index, line) in self.terms.lines().enumerate() {
            if index != 0 {
                terms += PrettyTree::Hardline
            }
            terms += "#? ";
            terms += line.to_owned();
        }
        terms.into()
    }
}
