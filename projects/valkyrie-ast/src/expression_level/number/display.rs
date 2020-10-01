use super::*;

impl PrettyPrint for NumberLiteralNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let num = theme.number(self.value.to_string());
        match &self.unit {
            Some(s) => {
                let unit = theme.metadata(s.name.to_string());
                num.append(unit)
            }
            None => num,
        }
    }
}
