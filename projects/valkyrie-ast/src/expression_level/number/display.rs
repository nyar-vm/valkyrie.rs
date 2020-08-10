use super::*;

impl PrettyPrint for NumberLiteralNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let num = allocator.number(self.value.to_string());
        match &self.unit {
            Some(s) => {
                let unit = allocator.metadata(s.name.to_string());
                num.append(unit)
            }
            None => num,
        }
    }
}
