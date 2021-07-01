use super::*;
use crate::ImportBlockNode;

impl crate::DefineImportNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ImportStatement> {
        match &self.import_block {
            None => {}
            Some(s) => {}
        }

        Ok(ImportStatement {
            annotation: Default::default(),
            term: ImportTermNode::Group(Box::new(ImportGroupNode { path: NamePathNode { names: vec![] }, group: vec![] })),
            span: self.span.clone(),
        })
    }
}

impl crate::ImportBlockNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ImportTermNode> {
        for term in self.import_term.iter() {}

        Ok(ImportTermNode::Group(Box::new(ImportGroupNode { path: NamePathNode { names: vec![] }, group: vec![] })))
    }
}

impl crate::ImportTermNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<Option<ImportTermNode>> {
        match self {
            Self::ImportAll(_) => {}
            Self::ImportMacro(_) => {}
            Self::ImportName(_) => {}
            Self::ImportSpace(_) => {}
            Self::EosFree(_) => None,
        }

        Ok(ImportTermNode::Group(Box::new(ImportGroupNode { path: NamePathNode { names: vec![] }, group: vec![] })))
    }
}
