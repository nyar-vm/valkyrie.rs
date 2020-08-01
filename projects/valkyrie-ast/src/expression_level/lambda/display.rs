use super::*;
use crate::PrettyTree;

impl PrettyPrint for LambdaCallNode {
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     self.base.indent_fmt(f)?;
    //     f.write_str(" {")?;
    //     f.indent();
    //     f.write_newline()?;
    //     for statement in &self.body {
    //         statement.indent_fmt(f)?;
    //     }
    //     f.dedent();
    //     f.write_newline()?;
    //     f.write_str("}")
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for LambdaDotNode {
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     self.base.indent_fmt(f)?;
    //     f.indent();
    //     f.write_newline()?;
    //     f.write_str(".{")?;
    //     f.indent();
    //     f.write_newline()?;
    //     for statement in &self.body {
    //         statement.indent_fmt(f)?;
    //     }
    //     f.dedent();
    //     f.write_newline()?;
    //     f.write_str("}")
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
