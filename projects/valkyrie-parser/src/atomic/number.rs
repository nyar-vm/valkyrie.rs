use super::*;
use crate::SpecialNode;

// A number literal.
// #[derive(Debug, Clone, Eq, Hash)]
// pub struct IntegerNode {}

//     ⍚(_*[0-9A-F])* # hex
// |   ⍙(_*[0-7])*       # octal
// |   ⍜(_*[01])*        # binary

impl SpecialNode {
    pub fn build(&self) -> BooleanNode {
        let value = match self.text.as_str() {
            "false" => false,
            "true" => true,
            _ => unimplemented!("Unknown boolean value: {}", self.text),
        };
        BooleanNode { value, span: self.span.clone() }
    }
}

impl crate::IntegerNode {
    pub fn build(&self) -> NumberLiteralNode {
        NumberLiteralNode { value: self.text.to_string(), unit: None, span: self.span.clone() }
    }
}
