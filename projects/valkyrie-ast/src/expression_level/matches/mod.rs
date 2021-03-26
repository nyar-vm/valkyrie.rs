use super::*;

/// `.match {}.catch {}`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MatchKind {
    /// Match pattern on type level
    Typing,
    /// Match pattern on effect level
    Effect,
}

/// `.match { when Some(a): a, else: 0}.catch { when IoError: (a), else: 0}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchDotStatement {
    /// expr?.match { }
    pub monadic: bool,
    /// The kind of the match statement
    pub kind: MatchKind,
    /// The patterns of the match statement
    pub patterns: PatternBlock,
    /// The range of the node
    pub span: Range<u32>,
}

impl MatchKind {
    /// Get the string representation of the match kind
    pub fn as_str(&self) -> &'static str {
        match self {
            MatchKind::Typing => "match",
            MatchKind::Effect => "catch",
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for MatchDotStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += PrettyTree::Hardline;
        terms += self.monadic.pretty(theme);
        terms += theme.keyword(self.kind.as_str());
        terms += " ";
        terms += self.patterns.pretty(theme);
        terms.into()
    }
}
