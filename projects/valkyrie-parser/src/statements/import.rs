use super::*;
use crate::ImportBlockNode;

impl crate::DefineImportNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ImportStatement> {
        Ok(ImportStatement { annotation: Default::default(), term: self.term(ctx), span: self.span.clone() })
    }
    fn term(&self, ctx: &mut ProgramState) -> ImportTermNode {
        if let Some(s) = self.import_term.and_then(|v| v.build(ctx)) {
            return s;
        }
        if let Some(s) = &self.import_block {
            return ImportGroupNode::from_iter(s.build(ctx)).into();
        }
        ImportTermNode::default()
    }
}

impl crate::ImportBlockNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Vec<ImportTermNode> {
        let mut terms = Vec::with_capacity(self.import_term.len());
        for term in self.import_term.iter() {
            if let Some(s) = term.build(ctx) {
                terms.push(s)
            }
        }
        terms
    }
}

impl crate::ImportTermNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Option<ImportTermNode> {
        match self {
            Self::ImportAll(v) => Some(v.build(ctx).into()),
            Self::ImportSpace(v) => Some(v.build(ctx).into()),
            Self::ImportName(v) => Some(v.build(ctx).into()),
            Self::EosFree(_) => None,
        }
    }
}

impl crate::ImportAllNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ImportAllNode {
        ImportAllNode { path: self.path.iter().map(|v| v.build(ctx)).collect() }
    }
}

impl crate::ImportSpaceNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ImportGroupNode {
        ImportGroupNode { path: self.path.iter().map(|v| v.build(ctx)).collect(), group: self.body.build(ctx) }
    }
}

impl crate::ImportNameNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ImportAliasNode {
        ImportGroupNode { path: self.path.iter().map(|v| v.build(ctx)).collect(), group: self.body.build(ctx) }
    }
}
impl crate::ImportAsNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Option<ImportAliasItem> {
        Some(self.alias?.build(ctx))
    }
}
