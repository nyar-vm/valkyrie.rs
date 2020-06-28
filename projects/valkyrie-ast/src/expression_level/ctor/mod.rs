use super::*;

/// `new kind Type<G>() {body}`
///
/// ```vk
/// new stack Type<G>(*args) {
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
pub struct NewStructureNode {
    pub modifiers: Vec<IdentifierNode>,
    pub arguments: Vec<ApplyTermNode<IdentifierNode, TermExpressionType>>,
    pub body: Vec<TermExpressionType>,
}

pub enum NewStructureCollectNode {}