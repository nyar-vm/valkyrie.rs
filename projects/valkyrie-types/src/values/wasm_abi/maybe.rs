use super::*;

impl ValkyrieValueType for OptionVal {
    fn as_valkyrie(&self) -> ValkyrieValue {
        match self.value() {
            Some(s) => ValkyrieMaybe::some(s).as_valkyrie(),
            None => ValkyrieMaybe::none(self.as_type()).as_valkyrie(),
        }
    }
    fn as_type(&self) -> ValkyrieType {
        self.ty().as_type()
    }
}

impl ValkyrieValueType for OptionType {
    fn as_valkyrie(&self) -> ValkyrieValue {
        unreachable!()
    }
    fn as_type(&self) -> ValkyrieType {
        // FIXME: missing option
        self.ty().as_type()
    }
}

impl ValkyrieValueType for ResultVal {
    fn as_valkyrie(&self) -> ValkyrieValue {
        todo!()
    }
    fn as_type(&self) -> ValkyrieType {
        self.ty().as_type()
    }
}
impl ValkyrieValueType for ResultType {
    fn as_valkyrie(&self) -> ValkyrieValue {
        unreachable!()
    }
    fn as_type(&self) -> ValkyrieType {
        unimplemented!()
    }
}
