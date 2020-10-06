use crate::{ArgumentKeyNode, ExpressionNode, StatementNode};
use alloc::{vec, vec::Vec};
use core::ops::Range;
use deriver::From;
use pretty_print::PrettyBuilder;
#[cfg(feature = "pretty-print")]
use pretty_print::{helpers::PrettySequence, PrettyPrint, PrettyProvider, PrettyTree};

mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternBranch {
    /// The condition of the branch
    pub condition: PatternCondition,
    /// The continuation of the branch
    pub statements: PatternStatements,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternStatements {
    pub terms: Vec<StatementNode>,
}

/// All valid branches of a pattern match statement
///
/// ```vk
/// expr
/// .match {
///     case Some(a) | Success { value: a }
///     when a > 0:
///         print("a is positive")
///     when a < 0:
///         print("a is negative")
///     type Integer | Decimal:
///         print("a is a number")
///     else:
///         print("a is not a number")
/// }
/// .catch {
///     case IoError(e):
///         print("IO Error: " + e.message)
///     else:
///         print("Unknown error")
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternCondition {
    /// `case Some(a) | Success { value :a }:`
    Case(PatternCaseNode),
    /// `when a > 0:`
    When(PatternWhenNode),
    /// `type Integer | Decimal:`
    Type(PatternTypeNode),
    /// `else:`
    Else(PatternElseNode),
}

/// `case Some(a) | Success { value :a } if a > 0:`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternCaseNode {
    pub pattern: ExpressionNode,
    pub guard: Option<PatternGuard>,
    /// The range of the node
    pub span: Range<u32>,
}

/// ```vk
/// let a, b = expr
/// let (a, b) = expr
/// let [a, b, **args] = expr
/// let Named(a, b) = expr
/// let Named {a, b, ***kws} = expr
/// let Named(Struct {a: b, b}, b, **args) = expr
///
/// for i in 0..10 if condition {
///     ...
/// }
/// else {
///     ...
/// }
/// ```
///
/// ```vk
/// let i = 1;
/// let j = 1;
/// let mut i, mut j;
/// let [a, b]
/// let (a, b)
/// ```
///
/// ```vk
/// for i in range;
/// for i, j in range;
/// for mut i, mut j in range
/// for [table] in
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternExpressionNode {
    Tuple(Vec<ArgumentKeyNode>),
    Case,
}

/// `Soma(a) | Success { value :a } := expr`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImplicitCaseNode {
    pub pattern: PatternExpressionNode,
    pub body: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// `when a > 0:`
///
/// ```vk
/// when a > 0:
/// case _ if a > 0:
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternWhenNode {
    pub guard: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// `type Integer | Decimal:`
///
///
/// ```vk
/// type Integer | Decimal:
/// case x if (x is (Integer | Decimal)):
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternTypeNode {
    pub pattern: ExpressionNode,
    pub guard: Option<PatternGuard>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `else:`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternElseNode {
    /// The range of the node
    pub span: Range<u32>,
}

/// `when a > 0 && a < 10`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternGuard {
    /// The post condition of the pattern
    pub condition: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

impl PrettyPrint for PatternGuard {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword("when").append(" ").append(self.condition.pretty(theme))
    }
}
