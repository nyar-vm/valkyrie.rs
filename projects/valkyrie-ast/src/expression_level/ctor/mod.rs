use super::*;
#[cfg(feature = "pretty-print")]
mod display;

/// `new stack Type⦓G⦔(args) { body }`
///
/// ```vk
/// new stack Type<G>(**args) {
///     value,
///     pair(key, value),
///     key: value,
///     [1]: value,
///     [1, 2:3]: body,
/// }
/// ```
///
/// ```vk
/// let body = new stack Type<G>(*args);
/// body.collect(value);
/// body.collect(pair(key, value));
/// body.key = value; # call setter
/// body[1] = value;  # call setter
/// body[1, 2:3] = value; # call setter
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewConstructNode {
    /// The modifiers for new
    pub modifiers: Vec<IdentifierNode>,
    pub namepath: NamePathNode,
    pub generic: GenericCallNode,
    pub arguments: ApplyCallNode,
    pub body: CollectsNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// `{ 1: x, p: y, [a, b]: c, **list, ***dict }`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectsNode {
    /// The collect terms
    pub terms: Vec<TableTermNode>,
    /// The range of the node
    pub span: Range<u32>,
}
