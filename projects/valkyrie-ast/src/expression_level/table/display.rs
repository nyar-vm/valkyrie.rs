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
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     write!(f, "{}", self.kind.begin_str())?;
    //     for (i, term) in self.terms.iter().enumerate() {
    //         if i != 0 {
    //             write!(f, ", ")?;
    //         }
    //         // term.indent_fmt(f)?;
    //     }
    //     write!(f, "{}", self.kind.end_str())
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
