use super::*;
use valkyrie_ast::{ImportGroupNode, ImportTermNode, NamePathNode};

impl crate::DefineImportNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ImportStatement> {
        Success {
            value: ImportStatement {
                annotation: Default::default(),
                term: ImportTermNode::Group(Box::new(ImportGroupNode { path: NamePathNode { names: vec![] }, group: vec![] })),
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
