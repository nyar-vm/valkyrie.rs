use super::*;

#[cfg(feature = "lispify")]
impl Lispify for ProgramRoot {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
