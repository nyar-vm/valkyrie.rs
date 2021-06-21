use super::*;
use crate::{StringElementNode, StringElementsNode};
use nyar_error::{Failure, NyarError, Result, Success, Validation};
use std::{mem::take, str::FromStr};
use valkyrie_ast::{
    ExpressionKind, IdentifierNode, StatementBlock, StatementNode, StringFormatted, StringFormattedTerm, StringInterpreter,
    StringLiteralNode, StringTextNode,
};

#[derive(Default)]
pub struct StringFormatterBuilder {
    terms: Vec<StringFormattedTerm>,
    nodes: Vec<StatementNode>,
    errors: Vec<NyarError>,
}

impl StringInterpreter for StringFormatterBuilder {
    type Output = StringFormatted;

    fn interpret(&mut self, text: &StringTextNode) -> Validation<Self::Output> {
        match StringElementsNode::from_str(&text.text) {
            Ok(o) => match o.build(self) {
                Ok(items) => Success {
                    value: StringFormatted { terms: items, span: text.span.clone() },
                    diagnostics: take(&mut self.errors),
                },
                Err(e) => Failure { fatal: e, diagnostics: take(&mut self.errors) },
            },
            Err(e) => Failure { fatal: NyarError::from(e), diagnostics: take(&mut self.errors) },
        }
    }
}

impl crate::StringElementsNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> StringFormatted {
        let mut list = StringFormatted::new(self.string_element.len(), &self.span);
        for x in &self.string_element {
            match x.build(ctx) {
                Ok(o) => list.terms.push(o),
                Err(e) => ctx.errors.push(e),
            }
        }
        list
    }
}
impl crate::StringElementNode {
    fn build(&self, ctx: &mut StringFormatterBuilder) -> Result<StringFormattedTerm> {
        match self {
            Self::EscapeCharacter(v) => {}
            Self::EscapeUnicode(v) => {}
            Self::StringInterpolationComplex(_) => {}
            Self::StringInterpolationSimple(_) => {}
        }
    }
}
