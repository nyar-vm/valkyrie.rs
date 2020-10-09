use core::ops::Range;
use pretty_print::{helpers::PrettySequence, PrettyBuilder, PrettyPrint, PrettyProvider, PrettyTree};

pub trait ValkyrieNode {
    fn get_range(&self) -> Range<u32>;
    // fn mut_range(&mut self) -> &mut Range<u32>;
}

/// A soft block is a block that is not required to be on a new line.
///
/// ```vk
/// {a, b, c}
///
/// {
///     a,
///     b,
/// }
/// ```
#[derive(Clone, Debug)]
pub struct SoftBlock {
    pub lhs: &'static str,
    pub rhs: &'static str,
    pub indent: usize,
    pub joint: PrettyTree,
}

impl SoftBlock {
    pub fn new(lhs: &'static str, rhs: &'static str) -> Self {
        Self { lhs, rhs, indent: 4, joint: PrettyTree::line_or_space() }
    }
    pub fn curly_braces() -> Self {
        Self::new("{", "}")
    }
    pub fn join_slice<T: PrettyPrint>(&self, slice: &[T], theme: &PrettyProvider) -> PrettyTree {
        let mut outer = PrettySequence::new(5);
        outer += self.lhs;
        outer += PrettyTree::line_or_space();
        let mut inner = PrettySequence::new(slice.len() * 2);
        for (idx, term) in slice.iter().enumerate() {
            if idx != 0 {
                inner += self.joint.clone();
            }
            inner += term.pretty(theme);
        }
        outer += inner.indent(self.indent);
        outer += PrettyTree::line_or_space();
        outer += self.rhs;
        outer.into()
    }
}
