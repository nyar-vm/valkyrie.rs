pub trait Evaluate {
    fn evaluate(&self, ctx: &mut ValkyrieInterpreter) -> Result<(), String>;
}

pub struct ValkyrieInterpreter {}
