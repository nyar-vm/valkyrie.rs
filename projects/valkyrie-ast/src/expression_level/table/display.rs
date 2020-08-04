use super::*;
use crate::PrettyTree;

impl TableKind {
    fn begin_str(&self) -> &'static str {
        match self {
            TableKind::Tuple => "(",
            TableKind::OffsetTable => "[",
            TableKind::OrdinalTable => "[",
        }
    }
    fn end_str(&self) -> &'static str {
        match self {
            TableKind::Tuple => ")",
            TableKind::OffsetTable => "]",
            TableKind::OrdinalTable => "]",
        }
    }
}

impl PrettyPrint for TableNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = allocator.text(self.kind.begin_str());
        let body = self.terms.iter().map(|x| x.build(allocator).append(allocator.text(",")));
        let tail = allocator.text(self.kind.end_str());
        head.append(allocator.concat(body)).append(tail)
    }
}

impl PrettyPrint for TableTermNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.pair.build(allocator)
    }
}

impl PrettyPrint for TableKeyKind {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            TableKeyKind::Identifier(node) => node.build(allocator),
            TableKeyKind::Number(node) => node.build(allocator),
            TableKeyKind::String(node) => node.build(allocator),
            TableKeyKind::Table(node) => node.build(allocator),
        }
    }
}
