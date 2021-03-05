use super::*;

// A number literal.
// #[derive(Debug, Clone, Eq, Hash)]
// pub struct IntegerNode {}

//     ⍚(_*[0-9A-F])* # hex
// |   ⍙(_*[0-7])*       # octal
// |   ⍜(_*[01])*        # binary

impl crate::BooleanNode {
    pub fn build(&self) -> BooleanNode {
        let value = match self {
            Self::False => false,
            Self::True => true,
        };
        BooleanNode { value, span: Default::default() }
    }
}

impl crate::IntegerNode {
    pub fn build(&self) -> NumberLiteralNode {
        NumberLiteralNode { value: self.text.to_string(), unit: None, span: self.span.clone() }
    }
}
