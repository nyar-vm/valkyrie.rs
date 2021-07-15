use super::*;
use crate::{helpers::FromFrontend, ValkyrieCodegen};
use valkyrie_ast::{ExpressionKind, FieldDeclaration};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldDefinition {
    symbol: Arc<str>,
    typing: Option<ExpressionKind>,
    span: FileSpan,
}

impl FromFrontend<FieldDefinition> for FieldDeclaration {
    fn build(&self, state: &mut ValkyrieCodegen) -> nyar_error::Result<FieldDefinition> {
        let mut output = FieldDefinition::new(&self.name);
        match &self.typing {
            Some(s) => output.set_type(s.clone()),
            None => {}
        }
        Ok(output)
    }
}

impl FieldDefinition {
    pub fn new(name: &IdentifierNode) -> Self {
        Self { symbol: Arc::from(name.name.as_str()), typing: None, span: Default::default() }
    }
    pub fn name(&self) -> String {
        self.symbol.to_string()
    }
    pub fn set_type(&mut self, typing: ExpressionKind) {
        self.typing = Some(typing);
    }
    pub fn get_type(&self) -> Option<&ExpressionKind> {
        self.typing.as_ref()
    }
    pub fn get_span(&self) -> FileSpan {
        self.span
    }
    pub fn set_span(&mut self, span: FileSpan) {
        self.span = span;
    }
}
