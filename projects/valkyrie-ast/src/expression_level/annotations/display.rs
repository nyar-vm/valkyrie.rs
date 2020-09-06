use super::*;

impl Display for AnnotationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for MacroPathNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     Display::fmt(&self.path, f)?;
    //     for item in &self.names {
    //         f.write_str(".")?;
    //         Display::fmt(item, f)?;
    //     }
    //     Ok(())
    // }

    fn build<'a>(&self, _allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
