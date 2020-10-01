use super::*;

impl<E: PrettyPrint> PrettyPrint for CallNode<E> {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let lhs = self.base.build(theme);
        let rhs = self.rest.build(theme);
        lhs.append(rhs)
    }
}

impl<K, V> PrettyPrint for CallTermNode<K, V>
where
    K: PrettyPrint,
    V: PrettyPrint,
{
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        if let Some(k) = &self.key {
            terms.push(k.build(theme));
            terms.push(theme.text(": "));
        }
        terms.push(self.value.build(theme));
        theme.concat(terms)
    }
}
