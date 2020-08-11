use super::*;

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
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("while"));
        terms.push(allocator.space());
        terms.push(self.condition.build(allocator));
        terms.push(FunctionBodyPart::build_borrowed(&self.body, allocator));
        allocator.concat(terms)
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

    fn build<'a>(&self, _allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
