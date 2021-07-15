use super::*;
use crate::helpers::IntoBackend;

impl ValkyrieCodegen {
    pub fn build(&mut self) -> Result<Vec<u8>> {
        self.module.build_module_wasm()
    }
    pub fn build_wasm<P: AsRef<Path>>(&mut self, file: P) -> Result<PathBuf> {
        let wasm = self.module.build_module_wasm();
        let path = file.as_ref();
        let dir = file.as_ref().parent().unwrap();
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file = std::fs::File::create(&path)?;
        file.write_all(&wasm?)?;
        Ok(path.canonicalize()?)
    }
    pub fn load_local<P: AsRef<Path>>(&mut self, path: P) -> Vec<NyarError> {
        let mut errors = take(&mut self.errors);
        if let Err(e) = self.try_load_local(path.as_ref()) {
            errors.push(e)
        }
        errors
    }
    fn try_load_local(&mut self, script: &Path) -> Result<()> {
        let file = self.cache.load_local(script)?;
        let ctx = ProgramContext { file };
        let root = ctx.parse(&mut self.cache).result(|e| self.errors.push(e))?;
        self.visit_ast(root)
    }
    fn visit_ast(&mut self, root: ProgramRoot) -> Result<()> {
        self.module.set_module_name("test");
        for x in &root.statements {
            match x {
                StatementKind::Nothing => {}
                StatementKind::Document(_) => {}
                StatementKind::Annotation(_) => {}
                StatementKind::Namespace(v) => self.current_namespace = v.path.clone(),
                StatementKind::Import(_) => {}
                StatementKind::Class(v) => {
                    let class = v.build(self)?.build(self)?;
                    self.module.insert_type(class);
                }
                StatementKind::Union(_) => {}
                StatementKind::Enumerate(_) => {}
                StatementKind::Trait(_) => {}
                StatementKind::Extends(_) => {}
                StatementKind::Function(_) => {}
                StatementKind::Variable(_) => {}
                StatementKind::Guard(_) => {}
                StatementKind::While(_) => {}
                StatementKind::For(_) => {}
                StatementKind::Control(_) => {}
                StatementKind::Expression(_) => {}
            }
        }
        Ok(())
    }
}

impl IntoBackend<StructureType> for ClassDefinition {
    fn build(&self, state: &mut ValkyrieCodegen) -> Result<StructureType> {
        let mut output = StructureType::new(Symbol::from(self.name()));

        for field in self.get_fields() {
            let field = field.build(state)?;
            output.add_field(field);
        }

        Ok(output)
    }
}

impl IntoBackend<FieldType> for FieldDefinition {
    fn build(&self, state: &mut ValkyrieCodegen) -> Result<FieldType> {
        let symbol = Symbol::new("wait");
        let mut output = FieldType::new(symbol);
        match self.get_type() {
            Some(ExpressionKind::Symbol(v)) => match v.to_string().as_str() {
                "u32" => output.r#type = NyarType::U32,
                "u64" => output.r#type = NyarType::I64,
                "i32" => output.r#type = NyarType::I32,
                "i64" => output.r#type = NyarType::I64,
                "f32" => output.r#type = NyarType::F32,
                "f64" => output.r#type = NyarType::F64,
                "Any" => output.r#type = NyarType::Any,
                "core::primitive::u32" => output.r#type = NyarType::U32,
                s => output.r#type = NyarType::Named { symbol: Symbol::new(s) },
            },
            Some(ExpressionKind::GenericCall(v)) => match &v.base {
                ExpressionKind::Symbol(s) => match s.to_string().as_str() {
                    "Array" => match &v.term {
                        GenericCallTerm::Associated(_) => {}
                        GenericCallTerm::Generic(g) => match g.terms.first() {
                            None => {}
                            Some(v) => match &v.value {
                                ExpressionKind::Symbol(v) => match v.to_string().as_str() {
                                    "u8" => output.r#type = NyarType::Array { inner: Box::new(NyarType::I8) },
                                    _ => {}
                                },
                                _ => {}
                            },
                        },
                    },
                    _ => {}
                },
                s => {
                    println!("UNKNOWN_FIELD_GENERIC: {s:?}")
                }
            },
            Some(s) => {
                println!("UNKNOWN_FIELD_TYPE: {s:?}")
            }
            None => {}
        }
        println!("{:#?}", output);
        Ok(output)
    }
}
