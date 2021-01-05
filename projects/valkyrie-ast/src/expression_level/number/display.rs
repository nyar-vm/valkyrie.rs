use super::*;

impl PrettyPrint for NumberLiteralNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let num = theme.number(self.value.to_string());
        match &self.unit {
            Some(s) => {
                let unit = theme.annotation(s.name.to_string());
                num.append(unit)
            }
            None => num,
        }
    }
}

#[cfg(feature = "lispify")]
impl Lispify for NumberLiteralNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let number = Lisp::number(self.value.clone());
        match &self.unit {
            Some(s) => number & Lisp::unit(s.name.clone()),
            None => number,
        }
    }
}
