use super::*;

/// `.match {}.catch {}`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MatchKind {
    Typing,
    Effect,
}

/// `.match { when Some(a): a, else: 0}.catch { when IoError: (a), else: 0}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchStatement {
    pub kind: MatchKind,
    pub branches: Vec<PatternBranch>,
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
