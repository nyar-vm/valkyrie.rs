use super::*;

mod codegen;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldDefinition {
    symbol: Arc<str>,
    typing: Option<ExpressionKind>,
    optional: bool,
    span: FileSpan,
}

impl FieldDefinition {
    pub fn new(name: &IdentifierNode) -> Self {
        Self { symbol: Arc::from(name.name.as_str()), typing: None, optional: false, span: Default::default() }
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
    pub fn get_optional(&self) -> bool {
        self.optional
    }
    pub fn set_optional(&mut self, optional: bool) {
        self.optional = optional;
    }
    pub fn get_span(&self) -> FileSpan {
        self.span
    }
    pub fn set_span(&mut self, span: FileSpan) {
        self.span = span;
    }
}
