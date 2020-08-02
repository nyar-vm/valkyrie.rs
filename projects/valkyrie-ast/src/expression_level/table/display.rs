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

impl<E: PrettyPrint> PrettyPrint for TableNode<E> {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = allocator.text(self.kind.begin_str());
        let body = self.terms.iter().map(|x| x.build(allocator).append(allocator.text(",")));
        let tail = allocator.text(self.kind.end_str());
        head.append(allocator.concat(body)).append(tail)
    }
}
