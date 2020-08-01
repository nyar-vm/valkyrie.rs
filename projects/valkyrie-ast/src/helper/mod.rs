use pretty::{
    termcolor::{Buffer, ColorSpec},
    Arena, DocBuilder,
};
use std::{ops::Deref, string::String};

pub type PrettyTree<'a> = DocBuilder<'a, Arena<'a, ColorSpec>, ColorSpec>;

pub struct PrettyProvider<'a> {
    arena: Arena<'a, ColorSpec>,
}

impl<'a> Deref for PrettyProvider<'a> {
    type Target = Arena<'a, ColorSpec>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl<'a> PrettyProvider<'a> {
    pub fn new() -> Self {
        PrettyProvider { arena: Arena::new() }
    }
}

pub trait PrettyPrint {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a>;
    fn pretty_string(&self, width: usize) -> String {
        let arena = PrettyProvider::new();
        let mut buffer = Buffer::ansi();
        if let Err(e) = self.pretty(&arena).render(width, &mut buffer) {
            return format!("Error: {}", e);
        }
        unsafe { String::from_utf8_unchecked(buffer.into_inner()) }
    }
    fn pretty_print(&self, width: usize) {
        let arena = PrettyProvider::new();
        let mut buffer = Buffer::ansi();
        match self.pretty(&arena).render_colored(width, &mut buffer) {
            Ok(_) => {
                println!("{}", unsafe { String::from_utf8_unchecked(buffer.into_inner()) });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
