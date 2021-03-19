use super::*;
use valkyrie_ast::NullNode;

// A number literal.
// #[derive(Debug, Clone, Eq, Hash)]
// pub struct IntegerNode {}

//     ⍚(_*[0-9A-F])* # hex
// |   ⍙(_*[0-7])*       # octal
// |   ⍜(_*[01])*        # binary

impl SpecialNode {
    pub fn build(&self) -> ExpressionType {
        match self.text.as_str() {
            "false" => BooleanNode { value: false, span: self.span.clone() }.into(),
            "true" => BooleanNode { value: true, span: self.span.clone() }.into(),
            "∞" => NullNode { nil: true, span: self.span.clone() }.into(),
            "∅" => NullNode { nil: true, span: self.span.clone() }.into(),
            _ => unimplemented!("Unknown special value: {}", self.text),
        }
    }
}

impl crate::IntegerNode {
    pub fn build(&self) -> NumberLiteralNode {
        NumberLiteralNode { value: self.text.to_string(), unit: None, span: self.span.clone() }
    }
}
