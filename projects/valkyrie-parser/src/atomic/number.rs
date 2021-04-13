use super::*;
use num_bigint::BigUint;
use nyar_error::NyarError;
use valkyrie_ast::NullNode;

// A number literal.
// #[derive(Debug, Clone, Eq, Hash)]
// pub struct IntegerNode {}

//     ⍚(_*[0-9A-F])* # hex
// |   ⍙(_*[0-7])*       # octal
// |   ⍜(_*[01])*        # binary

impl crate::NumberNode {
    pub fn build(&self, _: &ProgramContext) -> Result<ExpressionType, NyarError> {
        let n = match self {
            Self::Decimal(v) => v.build(),
            Self::DecimalX(v) => v.build(),
        };
        Ok(n.into())
    }
}

impl crate::DecimalXNode {
    pub fn build(&self) -> NumberLiteralNode {
        NumberLiteralNode::new(10, self.span.clone())
    }
}

impl crate::DecimalNode {
    pub fn build(&self) -> NumberLiteralNode {
        NumberLiteralNode::new(10, self.span.clone())
    }
}

impl crate::IntegerNode {
    pub fn build(&self) -> NumberLiteralNode {
        // let value = self.text.chars().filter(|c| c.is_digit(10)).collect();
        NumberLiteralNode::new(10, self.span.clone())
    }
}

impl SpecialNode {
    pub fn build(&self) -> ExpressionType {
        match self.text.as_str() {
            "false" => BooleanNode { value: false, span: self.span.clone() }.into(),
            "true" => BooleanNode { value: true, span: self.span.clone() }.into(),
            "∞" => NullNode { nil: true, span: self.span.clone() }.into(),
            "∅" | "nil" => NullNode { nil: true, span: self.span.clone() }.into(),
            _ => unimplemented!("Unknown special value: {}", self.text),
        }
    }
}
