use super::*;

mod display;

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
pub struct MatchStatement {
    /// The kind of the match statement
    pub kind: MatchKind,
    /// `match bind := expr { ... }`
    pub bind: Option<IdentifierNode>,
    /// `match expr { ... }`
    pub main: ExpressionKind,
    /// The patterns of the match statement
    pub patterns: PatternsList,
    /// The range of the node
    pub span: Range<u32>,
}

/// `.match { when Some(a): a, else: 0}.catch { when IoError: (a), else: 0}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchCallNode {
    /// expr?.match { }
    pub monadic: bool,
    /// The basement expression
    pub base: ExpressionKind,
    /// The kind of the match statement
    pub kind: MatchKind,
    /// The patterns of the match statement
    pub patterns: PatternsList,
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

impl MatchCallNode {
    /// Replace placeholder with actual expression
    pub fn with_base(self, base: ExpressionKind) -> Self {
        Self { base, ..self }
    }
}
