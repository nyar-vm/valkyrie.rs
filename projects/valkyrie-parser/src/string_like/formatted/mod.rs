use super::*;
use crate::{helpers::ProgramState, StringElementNode, StringElementsNode};
use nyar_error::{Failure, FileID, NyarError, Result, Success, SyntaxError, Validation};
use std::{mem::take, num::ParseIntError, ops::Range, str::FromStr};
use valkyrie_ast::{
    ExpressionKind, IdentifierNode, StatementBlock, StatementNode, StringFormatted, StringFormattedTerm, StringInterpreter,
    StringLiteralNode, StringTextNode,
};
use yggdrasil_rt::YggdrasilNode;

/// Build a formatted string
///
///
/// ```v
/// """
/// abc
/// \n\r\u{123}
/// {a}
/// {b:fmt}
/// {c, a: f}
/// """
/// ```
pub struct StringFormatterBuilder {
    file: FileID,
    buffer: StringTextNode,
    terms: Vec<StringFormattedTerm>,
    errors: Vec<NyarError>,
}

impl StringFormatterBuilder {
    pub fn new(file: FileID) -> Self {
        Self { file, buffer: Default::default(), terms: vec![], errors: vec![] }
    }
    fn extend_buffer(&mut self, range: &Range<u32>) {
        if self.buffer.text.is_empty() {
            self.buffer.span.start = range.start;
        }
        self.buffer.span.end = range.end;
    }
    fn push_buffer(&mut self) {
        if !self.buffer.text.is_empty() {
            let unescaped = take(&mut self.buffer);
            self.terms.push(StringFormattedTerm::Text { unescaped });
        }
    }
}

impl StringInterpreter for StringFormatterBuilder {
    type Output = StringFormatted;

    fn interpret(&mut self, text: &StringTextNode) -> Validation<Self::Output> {
        match StringElementsNode::from_str(&text.text) {
            Ok(o) => {
                let value = o.build(self);
                Success { value, diagnostics: take(&mut self.errors) }
            }
            Err(e) => Failure { fatal: NyarError::from(e).with_file(self.file), diagnostics: take(&mut self.errors) },
        }
    }
}

impl crate::StringElementsNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> StringFormatted {
        let mut list = StringFormatted::new(self.string_element.len(), &self.span);
        for x in self.string_element.iter() {
            if let Err(e) = x.build(ctx) {
                ctx.errors.push(e)
            }
        }
        ctx.push_buffer();
        list.terms = take(&mut ctx.terms);
        list
    }
}

impl crate::StringElementNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> Result<()> {
        match self {
            Self::EscapeCharacter(v) => v.build(ctx),
            Self::EscapeUnicode(v) => v.build(ctx),
            Self::StringInterpolationSimple(v) => v.build(ctx),
            Self::StringInterpolationComplex(v) => v.build(ctx),
            Self::StringInterpolationText(v) => v.build(ctx),
        }
    }
}
impl crate::StringInterpolationTextNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> Result<()> {
        ctx.extend_buffer(&self.span);
        ctx.buffer.text.push_str(&self.text);
        Ok(())
    }
}

impl crate::EscapeCharacterNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> Result<()> {
        match self.text.chars().last() {
            Some(c) => {
                ctx.extend_buffer(&self.span);
                match c {
                    'n' => ctx.buffer.text.push('\n'),
                    'r' => ctx.buffer.text.push('\r'),
                    s => ctx.buffer.text.push(s),
                }
                Ok(())
            }
            None => Err(NyarError::syntax_error("Invalid escape sequence", ctx.file.with_range(self.get_range()))),
        }
    }
}

impl crate::EscapeUnicodeNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> Result<()> {
        match u32::from_str(&self.code.text) {
            Ok(o) => match char::from_u32(o) {
                Some(s) => {
                    ctx.extend_buffer(&self.span);
                    ctx.buffer.text.push(s);
                    Ok(())
                }
                None => {
                    Err(SyntaxError::new("Unicode codepoint out of range").with_span(ctx.file.with_range(self.get_range())))?
                }
            },
            Err(e) => Err(NyarError::from(e).with_file(ctx.file)),
        }
    }
}

impl crate::StringInterpolationSimpleNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> Result<()> {
        ctx.push_buffer();
        self.main_expression.build(&mut ProgramState::new(ctx.file))?;

        Ok(())
    }
}

impl crate::StringInterpolationComplexNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> Result<()> {
        ctx.push_buffer();
        self.main_expression.build(&mut ProgramState::new(ctx.file))?;

        Ok(())
    }
}
