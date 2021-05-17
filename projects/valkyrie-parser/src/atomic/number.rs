use super::*;
use crate::SignNode;
use nyar_error::NyarError;
use std::str::FromStr;
use valkyrie_ast::NullNode;

// A number literal.
// #[derive(Debug, Clone, Eq, Hash)]
// pub struct IntegerNode {}

//     ⍚(_*[0-9A-F])* # hex
// |   ⍙(_*[0-7])*       # octal
// |   ⍜(_*[01])*        # binary

impl crate::NumberNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ExpressionType, NyarError> {
        let n = match self {
            Self::Decimal(v) => v.build(ctx)?,
            Self::DecimalX(v) => v.build(ctx)?,
        };
        Ok(n)
    }
}

impl crate::DecimalNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ExpressionType, NyarError> {
        let mut n = NumberLiteralNode::new(10, self.span.clone());
        n.set_integer(&self.lhs.text, ctx.file, self.lhs.span.start as usize)?;
        if let Some(s) = &self.rhs {
            n.set_decimal(&s.text, ctx.file, s.span.start as usize)?
        }
        if let Some(s) = &self.unit {
            n.unit = Some(s.build(ctx))
        }
        if let Some(s) = &self.shift {
            match &self.sign {
                Some(SignNode::Netative) => n.shift = -s.parse::<isize>(ctx)?,
                _ => n.shift = s.parse::<isize>(ctx)?,
            }
        }
        n.set_dot(self.dot.is_some());
        Ok(n.into())
    }
}

impl crate::DecimalXNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ExpressionType, NyarError> {
        let mut n = NumberLiteralNode::new(self.base.as_base(ctx)?, self.span.clone());
        n.set_integer(&self.lhs.text, ctx.file, self.lhs.span.start as usize)?;
        if let Some(s) = &self.rhs {
            n.set_decimal(&s.text, ctx.file, s.span.start as usize)?
        }
        if let Some(s) = &self.unit {
            n.unit = Some(s.build(ctx))
        }
        if let Some(s) = &self.shift {
            match &self.sign {
                Some(SignNode::Netative) => n.shift = -s.parse::<isize>(ctx)?,
                _ => n.shift = s.parse::<isize>(ctx)?,
            }
        }
        n.set_dot(self.dot.is_some());
        Ok(n.into())
    }
}

impl crate::IntegerNode {
    pub fn build(&self) -> NumberLiteralNode {
        NumberLiteralNode::new(10, self.span.clone())
    }
    pub fn as_identifier(&self, ctx: &mut ProgramState) -> IdentifierNode {
        let text = self.text.chars().filter(|c| c.is_digit(10)).collect();
        IdentifierNode { name: text, span: ctx.file.with_range(self.get_range()) }
    }
    pub fn as_base(&self, ctx: &mut ProgramState) -> Result<u32, NyarError> {
        let span = ctx.file.with_range(self.get_range());
        match u32::from_str(&self.text) {
            Ok(o) if o >= 2 && o <= 36 => Ok(o),
            Ok(_) => Err(NyarError::syntax_error(format!("Currently only `2 ⩽ base ⩽ 36` is supported"), span)),
            Err(e) => Err(NyarError::syntax_error(e.to_string(), span)),
        }
    }
    pub fn parse<T>(&self, ctx: &mut ProgramState) -> Result<T, NyarError>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error,
    {
        let span = ctx.file.with_range(self.get_range());
        match T::from_str(&self.text) {
            Ok(o) => Ok(o),
            Err(e) => Err(NyarError::syntax_error(e.to_string(), span)),
        }
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
