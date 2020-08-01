use super::*;
use crate::{FunctionBodyPart, PrettyPrint, PrettyProvider, PrettyTree};
use pretty::{DocAllocator, RefDoc};

impl PrettyPrint for WhileLoopNode {
    /// ```vk
    /// # inline style
    /// while a || b || c { ... }
    ///
    /// # block style
    /// while a
    ///     || b && c
    ///     && d || e
    /// {
    ///    ...
    /// }
    /// ```
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = allocator.keyword("while").append(allocator.space()).append(self.condition.pretty(allocator));
        let body = allocator.function_body(&self.body);
        head.append(body)
    }
}

impl PrettyPrint for ConditionType {
    /// ```vk
    /// # inline style
    /// a || b || c
    ///
    /// # block style
    ///
    /// a
    ///   || b && c
    ///   && d || e
    /// ```
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            ConditionType::AlwaysTrue => allocator.keyword("true"),
            ConditionType::Case => allocator.keyword("case"),
            ConditionType::Expression(e) => e.pretty(allocator),
        }
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
