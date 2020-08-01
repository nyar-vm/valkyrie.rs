use super::*;
use crate::PrettyTree;

impl PrettyPrint for ApplyDotNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        // self.base.indent_fmt(f)?;
        // f.write_newline()?;
        // write!(f, ".{}(", self.caller)?;
        // comma_terms(f, &self.terms)?;
        // write!(f, ")")
        todo!()
    }
}
impl<K, V> PrettyPrint for ApplyTermNode<K, V>
where
    K: PrettyPrint,
    V: PrettyPrint,
{
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        // match &self.key {
        //     Some(k) => {
        //         k.indent_fmt(f)?;
        //         f.write_str(": ")?;
        //         self.value.indent_fmt(f)
        //     }
        //     None => self.value.indent_fmt(f),
        // }
        todo!()
    }
}

impl PrettyPrint for ApplyCallNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        // self.base.indent_fmt(f)?;
        // write!(f, "(")?;
        // comma_terms(f, &self.terms)?;
        // write!(f, ")")
        todo!()
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
