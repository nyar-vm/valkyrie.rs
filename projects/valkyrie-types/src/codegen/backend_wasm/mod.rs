use super::*;
use nyar_wasm::{FieldType, NyarType, StructureType, Symbol};
use std::{
    io::Write,
    mem::take,
    path::{Path, PathBuf},
    str::FromStr,
};
use valkyrie_ast::{ArgumentTerm, ExpressionKind, GenericCallTerm};
use valkyrie_parser::{ClassTermNode, DefineClassNode, DefineFieldNode, ProgramNode, StatementNode, TypeExpressionNode};

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
    pub fn load_local<P: AsRef<Path>>(&mut self, path: P) -> Vec<NyarError> {
        let mut errors = take(&mut self.errors);
        if let Err(e) = self.try_load_local(path.as_ref()) {
            errors.push(e)
        }
        errors
    }
    fn try_load_local(&mut self, script: &Path) -> Result<()> {
        self.current_file = self.files.load_local(script)?;
        let text = self.files.fetch(&self.current_file)?.to_string();
        let root = ProgramNode::from_str(&text)?;
        self.visit_ast(root)
    }
    fn visit_ast(&mut self, root: ProgramNode) -> Result<()> {
        for x in &root.statement {
            match x {
                StatementNode::ControlFlow(_) => {}
                StatementNode::DefineClass(v) => {
                    let class = v.build(self)?;
                    self.module.insert_type(class);
                }
                StatementNode::DefineEnumerate(_) => {}
                StatementNode::DefineExtends(_) => {}
                StatementNode::DefineFunction(_) => {}
                StatementNode::DefineImport(_) => {}
                StatementNode::DefineNamespace(_) => {}
                StatementNode::DefineTrait(_) => {}
                StatementNode::DefineUnion(_) => {}
                StatementNode::DefineVariable(_) => {}
                StatementNode::Eos(_) => {}
                StatementNode::ExpressionRoot(_) => {}
                StatementNode::ForStatement(_) => {}
                StatementNode::WhileStatement(_) => {}
            }
        }
        Ok(())
    }
}

impl Codegen<StructureType> for DefineClassNode {
    fn build(&self, state: &mut ValkyrieWasmCodegen) -> Result<StructureType> {
        let name = self.identifier.build(state.current_file);
        let mut output = StructureType::new(Symbol::new(&name.name));
        for i in &self.class_block.class_term {
            match i {
                ClassTermNode::DefineDomain(_) => {}
                ClassTermNode::DefineField(v) => {
                    output.add_field(v.build(state)?);
                }
                ClassTermNode::DefineMethod(_) => {}
                ClassTermNode::EosFree(_) => {}
                ClassTermNode::ProceduralCall(_) => {}
            }
        }
        Ok(output)
    }
}

impl Codegen<FieldType> for DefineFieldNode {
    fn build(&self, state: &mut ValkyrieWasmCodegen) -> Result<FieldType> {
        let name = self.identifier.build(state.current_file);
        let mut output = FieldType::new(Symbol::new(&name.name));
        match &self.type_hint.hint {
            Some(s) => match s.build_external(state.current_file)? {
                ExpressionKind::Symbol(v) => match v.to_string().as_str() {
                    "u32" => output.r#type = NyarType::U32,
                    "u64" => output.r#type = NyarType::I64,
                    "i32" => output.r#type = NyarType::I32,
                    "i64" => output.r#type = NyarType::I64,
                    "f32" => output.r#type = NyarType::F32,
                    "f64" => output.r#type = NyarType::F64,
                    "Any" => output.r#type = NyarType::Any,
                    s => {
                        println!("UNKNOWN_FIELD_SYMBOL: {s}")
                    }
                },
                ExpressionKind::GenericCall(v) => match v.base {
                    ExpressionKind::Symbol(s) => match s.to_string().as_str() {
                        "Array" => match v.term {
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

                s => {
                    println!("UNKNOWN_FIELD_TYPE: {s:?}")
                }
            },
            None => {}
        }

        Ok(output)
    }
}

trait Codegen<Item> {
    fn build(&self, state: &mut ValkyrieWasmCodegen) -> Result<Item>;
}
