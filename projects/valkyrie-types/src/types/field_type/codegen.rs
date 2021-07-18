use super::*;

impl FromFrontend<FieldDefinition> for FieldDeclaration {
    fn build(&self, state: &mut ValkyrieCodegen) -> Result<FieldDefinition> {
        let mut output = FieldDefinition::new(&self.name);
        match &self.typing {
            Some(s) => output.set_type(s.clone()),
            None => {}
        }
        Ok(output)
    }
}

impl IntoBackend<FieldType> for FieldDefinition {
    fn build(&self, state: &mut ValkyrieCodegen) -> nyar_error::Result<FieldType> {
        let mut output = FieldType::new(Symbol::from(self.name()));
        output.mutable = true;
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
                s => output.r#type = NyarType::Named { symbol: Symbol::new(s), nullable: self.get_optional() },
            },
            Some(ExpressionKind::GenericCall(v)) => match &v.base {
                ExpressionKind::Symbol(s) => match s.to_string().as_str() {
                    "Array" => match &v.term {
                        GenericCallTerm::Associated(_) => {}
                        GenericCallTerm::Generic(g) => match g.terms.first() {
                            None => {}
                            Some(v) => match &v.value {
                                ExpressionKind::Symbol(v) => match v.to_string().as_str() {
                                    "u8" => output.r#type = NyarType::Array { inner: Box::new(NyarType::U8), nullable: false },
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
        Ok(output)
    }
}
