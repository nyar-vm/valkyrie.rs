mod display;

use crate::{ArgumentKey, ExpressionNode, IdentifierNode, ModifierList, NamePathNode, StatementNode};
use alloc::{boxed::Box, vec, vec::Vec};
use core::{
    fmt::{Debug, Formatter},
    ops::Range,
};
use deriver::From;
#[cfg(feature = "pretty-print")]
use pretty_print::{
    helpers::{PrettySequence, SoftBlock},
    PrettyBuilder, PrettyPrint, PrettyProvider, PrettyTree,
};

/// A pattern match statement block
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternBlock {
    /// Branches of the pattern match
    pub branches: Vec<PatternBranch>,
    /// The range of the node
    pub span: Range<u32>,
}

/// A pattern match branch
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

/// The continuation of a pattern branch
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternStatements {
    /// The statements of the branch
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
    /// `case bind <- Some(a)`
    pub pattern: ExpressionNode,
    /// `case a | b | c`
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
/// let i = 1;
/// let j = 1;
/// let mut i, mut j;
/// let [a, b]
/// let (a, b)
/// ```
///
/// ```vk
/// case Some(a)
///    | Success { value: a }
///    | Extractor { a, b: _, *** }
///    | [a, b: _, **arg, ***kws]
/// when a > 0
///    & a is Integer:
///     do something
/// ```
///
///
/// ```vk
/// for i in range;
/// for i, j in range;
/// for mut i, mut j in range
/// for [tuple] in
/// ```
#[derive(Clone, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternNode {
    /// `a, 'string', number, bool`
    Symbol(Box<ArgumentKey>),
    /// `(mut a, mut b)`
    Tuple(Box<TuplePatternNode>),
    /// `{ mut a: b, mut c: d }`
    Class(Box<ClassPatternNode>),
    /// `Some(a) | Success { value: a }`
    Union(Box<UnionPatternNode>),
    /// `[a, b, **]`
    Array(Box<ArrayPatternNode>),
    /// `#macro mod id`
    Atom(Box<IdentifierPattern>),
}

/// `#macro mod id`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierPattern {
    /// modifiers
    pub modifiers: ModifierList,
    /// identifiers
    pub identifier: IdentifierNode,
}

/// `case Some(a) | Success { value: a }:`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionPatternNode {
    /// `case bind <- Some(a)`
    pub bind: Option<IdentifierNode>,
    /// `case a | b | c`
    pub terms: Vec<PatternNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `bind := module∷Named(ref a, mut b, ..c)`
///
/// ```vk
/// match term {
///     case bind: module::Name(ref a, mut b, c) if a > 0:
///         body
/// }
/// ```
///
///
/// ```vk
/// let bind = term;
/// # Option<(?, ?, ?)>
/// let unapply = module::Name::extract(bind);
/// if unapply.is_none() {
///     break;
/// }
/// # (?, ?, ?)
/// let bind = unapply!;
/// let a = unapply!.0;
/// let b = unapply!.1;
/// let c = unapply!.2;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TuplePatternNode {
    /// `bind := ...`
    pub bind: Option<IdentifierNode>,
    /// `namespace::Name(...)`
    pub name: Option<NamePathNode>,
    /// `(ref a, mut b, ..c)`
    pub terms: Vec<PatternNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `a := Named { a: b, c: d }`
///
///
/// ```vk
/// match term {
///     case bind@module::Name {ref a, mut b, c} if a > 0:
///         body()
/// }
/// ```
///
///
/// ```vk
/// let bind = term;
/// # Option<(?, ?, ?)>
/// let unapply = module::Name::extract(bind);
/// if unapply.is_none() {
///     break;
/// }
/// # (?, ?, ?)
/// let bind = unapply!;
/// let a = unapply!.0;
/// let b = unapply!.1;
/// let c = unapply!.2;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassPatternNode {
    /// `bind <- ...`
    pub bind: Option<IdentifierNode>,
    /// `case namespace::Name()`
    pub name: Option<NamePathNode>,
    /// `case (ref a, mut b)`
    pub terms: Vec<PatternNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `bind <- [a, b, **]`
///
/// ```vk
/// match term {
///     case bind@module::Name[ref a, mut b, c] if a > 0:
///         body()
/// }
/// ```
///
///
/// ```vk
/// let bind = term;
/// # Option<(?, ?, ?)>
/// let unapply = module::Name::extract(bind);
/// if unapply.is_none() {
///     break;
/// }
/// # (?, ?, ?)
/// let bind = unapply!;
/// let a = unapply!.0;
/// let b = unapply!.1;
/// let c = unapply!.2;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayPatternNode {
    /// `bind <- ...`
    pub bind: Option<IdentifierNode>,
    /// `[a, b, **]`
    pub terms: Vec<PatternNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `Soma(a) | Success { value :a } := expr`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImplicitCaseNode {
    /// `Soma(a) | Success { value :a }`
    pub pattern: PatternNode,
    /// `:=`
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
    /// The condition of the when
    pub guard: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// `type Integer | Decimal:`
///
///
///
/// ```vk
/// type Integer | Decimal:
/// case x if (x is (Integer | Decimal)):
/// ```
///
///
/// ```vk
/// if x is Integer {
///
/// }
/// else if x is Decimal {
///
/// }
/// ````
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternTypeNode {
    /// The type of the pattern
    pub pattern: ExpressionNode,
    /// The range of the node
    pub guard: Option<PatternGuard>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `else: ...`
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
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternGuard {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword("when").append(" ").append(self.condition.pretty(theme))
    }
}
