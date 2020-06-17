use crate::ValkyrieExecutor;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use valkyrie_ast::{NamePathNode, NumberLiteralNode, StringLiteralNode};
use valkyrie_parser::expression::ValkyrieExpression;
use valkyrie_types::{ValkyrieClass, ValkyrieError, ValkyrieResult, ValkyrieValue};

pub struct ExecutorScope {
    parent: Option<Arc<Mutex<ExecutorScope>>>,
    variables: BTreeMap<String, ValkyrieValue>,
}

impl ValkyrieExecutor {
    pub(crate) async fn execute_expr(&mut self, expr: ValkyrieExpression) -> ValkyrieResult<ValkyrieValue> {
        match expr {
            ValkyrieExpression::Placeholder => Err(ValkyrieError::custom("Placeholder expression should never be executed")),
            ValkyrieExpression::Prefix(_) => {
                todo!()
            }
            ValkyrieExpression::Binary(_) => {
                todo!()
            }
            ValkyrieExpression::Suffix(_) => {
                todo!()
            }
            ValkyrieExpression::Number(v) => self.execute_number(*v).await,
            ValkyrieExpression::Symbol(v) => self.execute_symbol(*v).await,
            ValkyrieExpression::String(v) => self.execute_string(*v).await,
            ValkyrieExpression::Table(v) => {
                todo!()
            }
        }
    }
    pub(crate) async fn execute_number(&mut self, number: NumberLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match number.unit {
            Some(s) => match s.name.as_str() {
                "f32" => Ok(ValkyrieValue::Float32(number.value.parse::<f32>()?)),
                "f64" => Ok(ValkyrieValue::Float64(number.value.parse::<f64>()?)),
                _ => Err(ValkyrieError::custom(format!("Unknown unit: {}", s.name))),
            },
            None => match number.value.parse() {
                Ok(v) => Ok(ValkyrieValue::Integer(v)),
                Err(e) => Err(ValkyrieError::custom(format!("Unknown number: {}", e))),
            },
        }
    }
    pub(crate) async fn execute_symbol(&mut self, mut number: NamePathNode) -> ValkyrieResult<ValkyrieValue> {
        match number.names.len() {
            0 => Err(ValkyrieError::syntax_error("Unreachable empty symbol name", number.span)),
            1 => {
                let head = unsafe { number.names.pop().unwrap_unchecked() };
                match head.name.as_str() {
                    "true" => Ok(ValkyrieValue::Boolean(true)),
                    "false" => Ok(ValkyrieValue::Boolean(false)),
                    "null" => Ok(ValkyrieValue::Null),
                    _ => Err(ValkyrieError::custom(format!("Unknown symbol: {}", head.name))),
                }
            }
            _ => Err(ValkyrieError::custom(format!("Unknown symbol: {:?}", number.names))),
        }
    }
    pub(crate) async fn execute_string(&mut self, mut string: StringLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match string.unit {
            Some(s) => match s.name.as_str() {
                "re" => todo!(),
                "sh" => todo!(),
                "json" => self.execute_json(&string.value).await,
                _ => Err(ValkyrieError::custom(format!("Unknown handler: {}", s.name))),
            },
            // TODO: template string
            None => Ok(ValkyrieValue::UTF8String(Arc::new(string.value))),
        }
    }
    pub(crate) async fn execute_json(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
        // let value: Value = serde_json::from_str(string)?;
        // Ok(value)
        todo!()
    }
}
