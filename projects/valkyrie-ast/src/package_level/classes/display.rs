use super::*;

impl PrettyPrint for ClassDeclaration {
    // fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
    //     f.write_str("class")?;
    //
    //     f.write_char('}')
    // }

    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = allocator.keyword("class");
        let name = self.namepath.build(allocator);
        head.append(name)
    }
}
