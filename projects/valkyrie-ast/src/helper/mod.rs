use core::ops::Range;

pub trait ValkyrieNode {
    fn get_range(&self) -> Range<u32>;
    // fn mut_range(&mut self) -> &mut Range<u32>;
}
