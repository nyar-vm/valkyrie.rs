use super::*;

impl<E: PrettyPrint> PrettyPrint for CallNode<E> {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let lhs = self.base.pretty(theme);
        let rhs = self.rest.pretty(theme);
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
            terms += k.pretty(theme);
            terms += ": ";
        }
        terms += self.value.pretty(theme);
        terms.into()
    }
}
