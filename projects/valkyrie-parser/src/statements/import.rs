use super::*;

impl crate::DefineImportNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ImportStatement> {
        Ok(ImportStatement {
            annotation: Default::default(),
            term: ImportTermNode::Group(Box::new(ImportGroupNode { path: NamePathNode { names: vec![] }, group: vec![] })),
            span: self.span.clone(),
        })
    }
}
