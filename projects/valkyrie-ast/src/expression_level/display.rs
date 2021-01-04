use super::*;

impl Lispify for ExpressionNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        self.body.lispify()
    }
}

impl Lispify for ExpressionType {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
