use super::*;
use crate::PrettyTree;

impl PrettyPrint for NumberLiteralNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let num = allocator.text(self.value.to_string()).annotate(allocator.number_style());
        match &self.unit {
            Some(s) => {
                let unit = allocator.text(s.name.to_string()).annotate(allocator.macro_style());
                num.append(unit)
            }
            None => num,
        }
    }
}
