mod display;

use super::*;
use alloc::format;
use nyar_error::{NyarError, ReportKind, SyntaxError};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberLiteralNode {
    /// base representation of numbers
    pub base: u32,
    /// representation of input digits in base
    pub integer: String,
    /// representation of input digits in base
    pub decimal: String,
    /// Representation of input precision in base
    pub shift: isize,
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
        Self { base, integer: String::new(), decimal: String::new(), shift: 0, unit: None, span }
    }
    /// Set the digits of the number
    pub fn set_integer(&mut self, text: &str, file: FileID, start: usize) -> Result<(), NyarError> {
        self.integer = self.make_number(text, file, start)?;
        Ok(())
    }
    /// Set the digits of the number
    pub fn set_decimal(&mut self, text: &str, file: FileID, start: usize) -> Result<(), NyarError> {
        self.decimal = self.make_number(text, file, start)?;
        Ok(())
    }
    /// Set the precision of the number
    pub fn with_shift(self, shift: isize) -> Self {
        Self { shift, ..self }
    }
    /// Set the unit of the number
    pub fn with_unit(self, unit: IdentifierNode) -> Self {
        Self { unit: Some(unit), ..self }
    }

    fn make_number(&self, input: &str, file: FileID, start: usize) -> Result<String, NyarError> {
        let mut buffer = String::with_capacity(input.len());
        let mut delta = 0;
        for c in input.chars() {
            match c {
                '_' => {
                    delta += 1;
                    continue;
                }
                s => {
                    if s.is_digit(self.base) {
                        delta += c.len_utf8();
                        buffer.push(s);
                    }
                    else {
                        let error = SyntaxError {
                            info: "Invalid number literal".to_string(),
                            hint: format!("invalid text `{}` in base {}", s, self.base),
                            span: file.with_range(start + delta..start + delta + c.len_utf8()),
                        };
                        Err(error.as_error(ReportKind::Error))?
                    }
                }
            }
        }
        Ok(buffer)
    }
}
