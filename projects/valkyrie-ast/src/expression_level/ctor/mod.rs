use super::*;
use crate::{ApplyArgumentNode, PrettyTree, TableTermNode};

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
pub struct NewStructureNode {
    pub modifiers: Vec<IdentifierNode>,
    pub namepath: NamePathNode,
    pub generic: GenericCallNode,
    pub arguments: ApplyArgumentNode,
    pub collectors: Vec<TableTermNode>,
}

impl PrettyPrint for NewStructureNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(5);
        terms.push(allocator.keyword("new"));
        for m in &self.modifiers {
            terms.push(allocator.space());
            terms.push(m.build(allocator));
        }
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));

        if !self.generic.terms.is_empty() {
            terms.push(self.generic.build(allocator));
        }
        terms.push(self.arguments.build(allocator));
        if !self.collectors.is_empty() {
            let head = allocator.text("{");
            let body = self.collectors.iter().map(|x| x.build(allocator).append(allocator.text(",")));
            let tail = allocator.text("}");
            let table = head.append(allocator.concat(body)).append(tail);
            terms.push(table)
        }
        allocator.concat(terms)
    }
}
