use super::*;
use pretty_print::KAndRBracket;

impl PrettyPrint for TableNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self.kind {
            TableKind::Tuple => {
                let k = KAndRBracket { head_space: false, bracket_l: "(", bracket_r: ")" };
                k.build(&self.terms, allocator, allocator.text(", "), allocator.text(",").append(allocator.hardline()))
            }
            TableKind::OffsetTable => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, allocator, allocator.text(", "), allocator.text(",").append(allocator.hardline()))
            }
            TableKind::OrdinalTable => {
                let k = KAndRBracket { head_space: false, bracket_l: "{%", bracket_r: "%}" };
                k.build(&self.terms, allocator, allocator.text(", "), allocator.text(",").append(allocator.hardline()))
            }
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for TableTermNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.pair.build(allocator)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for TableKeyType {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            TableKeyType::Identifier(node) => node.build(allocator),
            TableKeyType::Number(node) => node.build(allocator),
            TableKeyType::String(node) => node.build(allocator),
            TableKeyType::Subscript(node) => node.build(allocator),
        }
    }
}
