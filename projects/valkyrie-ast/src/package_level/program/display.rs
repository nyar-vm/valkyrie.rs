use crate::ProgramRoot;
use lispify::{Lisp, Lispify};

impl Lispify for ProgramRoot {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
