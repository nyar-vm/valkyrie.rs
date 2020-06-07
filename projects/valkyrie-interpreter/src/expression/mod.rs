use crate::ValkyrieExecutor;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use valkyrie_ast::{IdentifierNode, NamePathNode, NumberLiteralNode};
use valkyrie_parser::expression::ValkyrieExpression;
use valkyrie_types::{ValkyrieError, ValkyrieResult, ValkyrieValue};

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
            ValkyrieExpression::String(_) => {
                todo!()
            }
            ValkyrieExpression::Table(_) => {
                todo!()
            }
        }
    }
    pub(crate) async fn execute_number(&mut self, number: NumberLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match number.unit {
            Some(s) => match s.name.as_str() {
                "u8" => Ok(ValkyrieValue::Unsigned8(number.value.parse::<u8>()?)),
                "u16" => Ok(ValkyrieValue::Unsigned16(number.value.parse::<u16>()?)),
                "u32" => Ok(ValkyrieValue::Unsigned32(number.value.parse::<u32>()?)),
                "u64" => Ok(ValkyrieValue::Unsigned64(number.value.parse::<u64>()?)),
                _ => Err(ValkyrieError::custom(format!("Unknown unit: {}", s.name))),
            },
            None => match number.value.parse::<i64>() {
                Ok(v) => Ok(ValkyrieValue::Integer64(v)),
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
}
