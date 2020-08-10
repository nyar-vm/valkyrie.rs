use crate::IdentifierNode;
use alloc::{borrow::Cow, string::String};
use core::ops::{Deref, Range};

pub trait ValkyrieNode {
    fn get_range(&self) -> Range<u32>;
    // fn mut_range(&mut self) -> &mut Range<u32>;
}
