use super::*;

impl Display for AnnotationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PrettyPrint for AnnotationKind {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.metadata(self.as_str())
    }
}

impl PrettyPrint for AnnotationNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for AnnotationPathNode {
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

impl PrettyPrint for AnnotationList {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
