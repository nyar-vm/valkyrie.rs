use super::*;
use crate::PrettyTree;

impl PrettyPrint for ApplyDotNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.pretty(allocator);
        let lhs = allocator
            .text(".")
            .append(allocator.text(self.caller.name.clone()))
            .append(allocator.text("("))
            .append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text(")"));
        let body = self.terms.iter().map(|x| x.pretty(allocator).append(allocator.softline()));
        head.append(lhs).append(allocator.concat(body)).append(rhs)
    }
}
impl<K, V> PrettyPrint for ApplyTermNode<K, V>
where
    K: PrettyPrint,
    V: PrettyPrint,
{
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match &self.key {
            Some(k) => {
                let lhs = k.pretty(allocator).append(allocator.text(": "));
                let rhs = self.value.pretty(allocator);
                lhs.append(rhs)
            }
            None => self.value.pretty(allocator),
        }
    }
}

impl PrettyPrint for ApplyCallNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        // self.base.indent_fmt(f)?;
        // write!(f, "(")?;
        // comma_terms(f, &self.terms)?;
        // write!(f, ")")

        let head = self.base.pretty(allocator);
        let lhs = allocator.text("(").append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text(")"));
        let body = self.terms.iter().map(|x| x.pretty(allocator).append(allocator.softline()));
        head.append(lhs).append(allocator.concat(body)).append(rhs)
    }
}

impl PrettyPrint for ArgumentKeyNode {
    // for modifier in &self.modifiers {
    //     write!(f, "{} ", modifier)?;
    // }
    // write!(f, "{}", self.key)
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
