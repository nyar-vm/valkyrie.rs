use core::ops::Range;
use pretty_print::{helpers::PrettySequence, PrettyBuilder, PrettyPrint, PrettyProvider, PrettyTree};

pub trait ValkyrieNode {
    fn get_range(&self) -> Range<u32>;
    // fn mut_range(&mut self) -> &mut Range<u32>;
}
