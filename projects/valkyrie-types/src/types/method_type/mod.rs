use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodDefinition {
    symbol: Vec<Arc<str>>,
    typing: Option<ExpressionKind>,
    span: FileSpan,
}

impl FromFrontend<MethodDefinition> for MethodDeclaration {
    fn build(&self, state: &mut ValkyrieCodegen) -> nyar_error::Result<MethodDefinition> {
        todo!()
    }
}

impl MethodDefinition {
    pub fn new(name: &IdentifierNode) -> Self {
        todo!()
    }
    pub fn name(&self) -> String {
        todo!()
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
}
