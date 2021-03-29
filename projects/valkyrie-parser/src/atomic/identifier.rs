use super::*;

impl crate::NamepathNode {
    pub fn build(&self, ctx: &ProgramContext) -> NamePathNode {
        NamePathNode { names: self.identifier.iter().map(|v| v.build(ctx)).collect() }
    }
}

impl crate::NamepathFreeNode {
    pub fn build(&self, ctx: &ProgramContext) -> NamePathNode {
        NamePathNode { names: self.identifier.iter().map(|v| v.build(ctx)).collect() }
    }
}
impl crate::IdentifierNode {
    pub fn build(&self, ctx: &ProgramContext) -> IdentifierNode {
        match self {
            Self::IdentifierBare(v) => IdentifierNode { name: v.text.to_string(), span: ctx.file.with_range(v.get_range()) },
            Self::IdentifierRaw(v) => {
                IdentifierNode { name: v.identifier_raw_text.text.to_string(), span: ctx.file.with_range(v.get_range()) }
            }
        }
    }
}
