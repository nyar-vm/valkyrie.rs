use super::*;

impl ResolveContext {
    pub fn parse(&mut self, file: FileID, cache: &mut FileCache) -> Vec<NyarError> {
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

    pub fn visit(&mut self, root: ProgramRoot) -> Vec<NyarError> {
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
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        for statement in self.statements {
            statement.to_mir(ctx)?
        }
        Ok(())
    }
}

impl Hir2Mir for StatementKind {
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
            Self::Class(v) => {
                let item = v.to_mir(ctx)?;
            }
            Self::Union(v) => {
                let variant = v.to_mir(ctx)?;
                ctx.items.insert(variant.symbol.clone(), ModuleItem::Variant(variant));
            }
            Self::Enumerate(_) => {
                todo!()
            }
            Self::Trait(_) => {
                todo!()
            }
            Self::Extends(_) => {
                todo!()
            }
            Self::Function(v) => {
                let fun = v.to_mir(ctx)?;
            }
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

impl Hir2Mir for FunctionDeclaration {
    type Output = ModuleItem;
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        Ok(ModuleItem::External(ValkyrieExternalFunction {}))
    }
}

impl Hir2Mir for NamespaceDeclaration {
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        ctx.namespace = Some(self.path);
        Ok(())
    }
}
