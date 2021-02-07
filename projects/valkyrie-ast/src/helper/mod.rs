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
