use super::*;
use crate::ValkyrieClassType;
use nyar_wasm::{FieldType, StructureType, Symbol};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use valkyrie_ast::{ClassTerm, ExpressionKind, ExpressionNode};

impl ValkyrieWasmCodegen {
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
    pub fn load_module(&mut self, script: Url) -> Vec<NyarError> {
        self.try_load_module(script).err().into_iter().collect()
    }
    fn try_load_module(&mut self, script: Url) -> Result<()> {
        let file = self.files.load_remote(script)?;
        let parser = ProgramContext { file };
        match parser.parse(&mut self.files) {
            Success { value, diagnostics } => {
                if diagnostics.is_empty() {
                    self.visit_ast(value)
                }
                else {
                    self.errors.extend(diagnostics);
                    Err(NyarError::runtime_error("parse failed"))
                }
            }
            Failure { fatal, diagnostics } => {
                self.errors.extend(diagnostics);
                Err(fatal)
            }
        }
    }
    fn visit_ast(&mut self, root: ProgramRoot) -> Result<()> {
        for x in root.statements {
            match x {
                StatementKind::Nothing => {}
                StatementKind::Document(_) => {}
                StatementKind::Annotation(_) => {}
                StatementKind::Namespace(_) => {}
                StatementKind::Import(_) => {}
                StatementKind::Class(v) => {
                    let mut fields = vec![];
                    println!("Class: {:?}", v.name.name);

                    for i in v.terms {
                        match i {
                            ClassTerm::Macro(_) => {}
                            ClassTerm::Field(v) => {
                                match v.typing {
                                    Some(ExpressionKind::Symbol(v)) => match v.to_string().as_str() {
                                        s => {
                                            println!("{s}")
                                        }
                                        _ => {}
                                    },
                                    _ => panic!(),
                                }

                                fields.push(FieldType::new(Symbol::new(&v.name.name)));
                            }
                            ClassTerm::Method(_) => {}
                            ClassTerm::Domain(_) => {}
                        }
                    }

                    self.module.insert_type(StructureType::new(Symbol::new(&v.name.name)).with_fields(fields));
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
