use crate::{ExpressionBody, FormatterType, IdentifierNode, StringLiteralNode};
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use core::ops::Range;

pub mod string_template;

impl StringLiteralNode {
    pub fn new<S: ToString>(value: S, start: u32, end: u32) -> Self {
        Self { value: value.to_string(), unit: None, span: start..end }
    }
    pub fn as_raw(&self) -> String {
        self.value.to_owned()
    }
    pub fn as_escaped(&self) -> String {
        self.to_string()
    }
    pub fn with_unit(mut self, unit: IdentifierNode) -> Self {
        self.unit = Some(unit);
        self
    }
}
