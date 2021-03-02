use crate::{ValkyrieType, ValkyrieValue, ValkyrieValueType};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ValkyrieMaybe {
    ok: bool,
    value: Option<ValkyrieValue>,
    lhs_type: Option<ValkyrieType>,
    rhs_type: Option<ValkyrieType>,
}

impl ValkyrieMaybe {
    pub fn value<V>(v: V, rhs: ValkyrieType) -> ValkyrieMaybe
    where
        V: ValkyrieValueType,
    {
        Self { ok: true, value: Some(v.as_valkyrie()), lhs_type: Some(v.as_type()), rhs_type: Some(rhs) }
    }
    pub fn error<V>(v: V, lhs: ValkyrieType) -> ValkyrieMaybe
    where
        V: ValkyrieValueType,
    {
        Self { ok: false, value: Some(v.as_valkyrie()), lhs_type: Some(lhs), rhs_type: Some(v.as_type()) }
    }
    pub fn some<V>(v: &V) -> ValkyrieMaybe
    where
        V: ValkyrieValueType,
    {
        Self { ok: true, value: Some(v.as_valkyrie()), lhs_type: Some(v.as_type()), rhs_type: None }
    }
    pub fn none(lhs: ValkyrieType) -> ValkyrieMaybe {
        Self { ok: false, value: None, lhs_type: Some(lhs), rhs_type: None }
    }
    pub fn null() -> ValkyrieMaybe {
        Self { ok: false, value: None, lhs_type: None, rhs_type: None }
    }

    pub fn as_option(&self) -> Option<ValkyrieValue> {
        todo!()
    }
    pub fn as_result(&self) -> Result<ValkyrieValue, ValkyrieValue> {
        todo!()
    }
}

impl ValkyrieValueType for ValkyrieMaybe {
    fn as_valkyrie(&self) -> ValkyrieValue {
        ValkyrieValue::Maybe(Box::new(self.clone()))
    }
    fn as_type(&self) -> ValkyrieType {
        todo!()
    }
}
