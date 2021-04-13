mod display;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberLiteralNode {
    /// base representation of numbers
    pub base: u32,
    /// representation of input digits in base
    pub digits: String,
    /// Representation of input precision in base
    pub significant: Option<String>,
    /// unit of the input number
    pub unit: Option<IdentifierNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl ValkyrieNode for NumberLiteralNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl NumberLiteralNode {
    /// Create a new number with given base
    pub fn new(base: u32, span: Range<u32>) -> Self {
        Self { base, digits: String::new(), significant: None, unit: None, span }
    }
    /// Set the digits of the number
    pub fn with_digits(self, text: &str) -> Self {
        let digits = text.chars().filter(|c| c.is_digit(self.base)).collect();
        Self { digits, ..self }
    }
    /// Set the precision of the number
    pub fn with_significant(self, text: &str) -> Self {
        // 0xFF|¦1.6
        let significant = text.chars().filter(|c| c.is_digit(10)).collect();
        Self { significant: Some(significant), ..self }
    }
    /// Set the unit of the number
    pub fn with_unit(self, unit: IdentifierNode) -> Self {
        Self { unit: Some(unit), ..self }
    }
}
