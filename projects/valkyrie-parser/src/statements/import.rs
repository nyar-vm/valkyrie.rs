use super::*;

impl crate::DefineImportNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ImportStatement> {
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
