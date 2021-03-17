use core::fmt::{Debug, Display, Formatter};
#[doc = include_str!("readme.md")]
use core::ops::Range;

#[cfg(feature = "lispify")]
pub use lispify::{Lisp, Lispify};
#[cfg(feature = "pretty-print")]
pub use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

/// A node in the AST
pub trait ValkyrieNode {
    /// The range of the node
    fn get_range(&self) -> Range<usize>;
    // fn mut_range(&mut self) -> &mut Range<u32>;
    /// Get the start of the node
    fn get_start(&self) -> usize {
        self.get_range().start
    }
    /// Get the end of the node
    fn get_end(&self) -> usize {
        self.get_range().end
    }
}

pub(crate) struct WrapDisplay<'a, T> {
    inner: &'a T,
}
impl<'a, T> WrapDisplay<'a, T> {
    pub fn new(wrap: &'a T) -> Self {
        Self { inner: wrap }
    }
}

impl<'a, T: Display> Debug for WrapDisplay<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self.inner, f)
    }
}
