use super::*;
use crate::{PrettyPrint, PrettyProvider, PrettyTree};

impl PrettyPrint for WhileLoopNode {
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     write!(f, "while {} {{", self.condition)?;
    //     f.indent();
    //     for node in self.body.iter() {
    //         f.write_newline()?;
    //         node.indent_fmt(f)?;
    //     }
    //     f.dedent();
    //     f.write_newline()?;
    //     f.write_char('}')?;
    //     format_else_body(f, &self.r#else)
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for ForLoopNode {
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     for i in &self.body {
    //         write!(f, "{}\n", i)?;
    //     }
    //     f.write_newline()?;
    //     f.write_char('}')?;
    //     format_else_body(f, &self.r#else)
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
