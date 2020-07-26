use alloc::string::{String, ToString};
use pretty::{termcolor::ColorSpec, Arena, RcDoc, RefDoc};

pub trait PrettyPrint {
    fn pretty(&self, allocator: Arena) -> RefDoc<ColorSpec>;
    fn pretty_string(&self, width: usize) -> String {
        let arena = Arena::new();
        self.pretty(arena).pretty(width).to_string()
    }
    fn pretty_print(&self, width: usize) {
        let arena = Arena::new();
        println!("{}", self.pretty(arena).pretty(80).to_string());
    }
}

impl LispNumber {
    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        use pretty::{docs, Arena, DocAllocator};
        let arena = &Arena::<()>::new();
        let doc = docs![arena, "let", arena.softline(), "x", arena.softline(), "=", arena.softline(), Some("123"),];
        assert_eq!(doc.1.pretty(80).to_string(), "let x = 123");

        let n = RcDoc::text(&self.number).annotate(ColorSpec::new().set_fg(Some(Color::Red)).clone());
        if self.unit.is_empty() {
            n
        }
        else {
            let unit = RcDoc::text(&self.unit).annotate(ColorSpec::new().set_fg(Some(Color::Cyan)).clone());
            n.append(unit)
        }
    }
}
