use super::*;

impl ResolveContext {
    /// Parse a fetch text from the source cache
    pub fn parse(&mut self, file: SourceID, cache: &mut SourceCache) -> Vec<NyarError> {
        let root = ProgramContext { file }.parse(cache);
        let mut errors = vec![];
        match root {
            Success { value, diagnostics } => {
                errors.extend(diagnostics);
                errors.extend(self.visit(value))
            }
            Failure { fatal, diagnostics } => {
                errors.extend(diagnostics);
                errors.extend_one(fatal);
            }
        }
        errors
    }

    fn visit(&mut self, root: ProgramRoot) -> Vec<NyarError> {
        let progress = root.to_mir(self);
        let mut errors = take(&mut self.errors);
        match progress {
            Ok(_) => {}
            Err(e) => errors.push(e),
        }
        errors
    }
}

impl Hir2Mir for ProgramRoot {
    type Output = ();

    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        for statement in self.statements {
            statement.to_mir(ctx)?
        }
        Ok(())
    }
}

impl Hir2Mir for StatementKind {
    type Output = ();

    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        match self {
            Self::Nothing => {
                todo!()
            }
            Self::Document(_) => {
                todo!()
            }
            Self::Annotation(_) => {
                todo!()
            }
            Self::Namespace(v) => v.to_mir(ctx)?,
            Self::Import(_) => {
                todo!()
            }
            Self::Class(v) => v.to_mir(ctx)?,
            Self::Union(v) => v.to_mir(ctx)?,
            Self::Enumerate(_) => {
                todo!()
            }
            Self::Trait(_) => {
                todo!()
            }
            Self::Extends(_) => {
                todo!()
            }
            Self::Function(v) => v.to_mir(ctx)?,
            Self::Variable(_) => {
                todo!()
            }
            Self::Guard(_) => {
                todo!()
            }
            Self::While(_) => {
                todo!()
            }
            Self::For(_) => {
                todo!()
            }
            Self::Control(_) => {
                todo!()
            }
            Self::Expression(_) => {
                todo!()
            }
        }
        Ok(())
    }
}

impl Hir2Mir for NamespaceDeclaration {
    type Output = ();

    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        ctx.namespace.clear();
        match self.path.path.as_slice() {
            // clear current namespace
            [head] if head.name.as_ref().eq("_") => {}
            [head, rest @ ..] => {
                match head.name.as_ref().eq("package") {
                    true => ctx.namespace.push(ctx.package.clone()),
                    false => ctx.namespace.push(head.name.clone()),
                }
                for x in rest {
                    ctx.namespace.push(x.name.clone())
                }
            }
            _ => {}
        }
        Ok(())
    }
}
