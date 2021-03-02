use super::*;
use crate::values::ValkyrieType;

impl ValkyrieValueType for bool {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Boolean(*self)
    }
    fn as_type(&self) -> ValkyrieType {
        ValkyrieType::Boolean
    }
}

// pub struct ValkyrieFunction {
//     document: String,
//     parameters: Vec<ValkyrieMetaType>,
//     return_type: ValkyrieMetaType,
//     function_ptr: ValkyrieFunctionInstance,
// }
//
// pub enum ValkyrieFunctionInstance {
//     Normal { apply: fn(Vec<ValkyrieValue>) -> ValkyrieValue },
//     Curry { apply: fn(Vec<ValkyrieValue>) -> ValkyrieValue, parameters: Vec<ValkyrieValue> },
// }

pub fn and(p: bool, q: bool) -> bool {
    p && q
}

pub fn or(p: bool, q: bool) -> bool {
    p || q
}

pub fn logic_gate(p: bool, q: bool, mask: u8) -> Result<bool, String> {
    let ok = match mask {
        0 => false,
        1 => p && q,
        2 => p && !q,
        3 => p,
        4 => !p && q,
        5 => q,
        6 => p ^ q,
        7 => p || q,
        8 => !p && !q,
        9 => p == q,
        10 => !q,
        11 => p || !q,
        12 => !p,
        13 => !p || q,
        14 => !p || !q,
        15 => true,
        _ => return Err(format!("Invalid mask: {}", mask)),
    };
    Ok(ok)
}
