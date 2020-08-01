use super::*;
use crate::PrettyTree;

impl PrettyPrint for GenericCall {
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     write!(f, "{}", self.base)?;
    //     write!(f, "⦓")?;
    //     // comma_terms(f, &self.terms)?;
    //     write!(f, "⦔")
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for GenericArgumentNode {
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     write!(f, "⦓")?;
    //     comma_terms(f, &self.terms)?;
    //     write!(f, "⦔")
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
