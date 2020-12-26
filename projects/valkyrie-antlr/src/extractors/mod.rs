use std::rc::Rc;

pub trait Extractor<R>
where
    Self: Sized,
{
    fn take(node: Option<Rc<R>>) -> Option<Self>;

    fn take_many(nodes: Vec<Rc<R>>) -> Vec<Self> {
        let mut out = Vec::with_capacity(nodes.len());
        for node in nodes {
            if let Some(s) = Self::take(Some(node)) {
                out.push(s);
            }
        }
        out
    }
}
