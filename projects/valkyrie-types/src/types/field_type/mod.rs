use super::*;
use valkyrie_ast::ExpressionKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldDefinition {
    symbol: Arc<str>,
    typing: Option<ExpressionKind>,
    span: FileSpan,
}

impl FieldDefinition {
    pub fn new(name: &IdentifierNode) -> Self {
        Self { symbol: Arc::from(name.name.as_str()), typing: None, span: Default::default() }
    }
    pub fn set_type(&mut self, typing: ExpressionKind) {
        self.typing = Some(typing);
    }
    pub fn get_type(&self) -> Option<&ExpressionKind> {
        self.typing.as_ref()
    }
}
