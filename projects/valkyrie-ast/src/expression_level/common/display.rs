use super::*;

impl<E: PrettyPrint> PrettyPrint for CallNode<E> {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let lhs = self.base.build(allocator);
        let rhs = self.rest.build(allocator);
        lhs.append(rhs)
    }
}

impl<K, V> PrettyPrint for CallTermNode<K, V>
where
    K: PrettyPrint,
    V: PrettyPrint,
{
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        if let Some(k) = &self.key {
            terms.push(k.build(allocator));
            terms.push(allocator.text(": "));
        }
        terms.push(self.value.build(allocator));
        allocator.concat(terms)
    }
}
