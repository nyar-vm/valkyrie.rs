use std::str::Chars;

pub struct StringRewrite<'a> {
    chars: Chars<'a>,
}

impl<'a> StringRewrite<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { chars: s.chars() }
    }
    pub fn view(s: &'a str, start: usize, end: usize) -> Self {
        debug_assert!(start <= end, "start must be less than or equal to end");
        debug_assert!(end <= s.len(), "end must be less than or equal to the length of the string");
        unsafe { Self { chars: s.get_unchecked(start..end).chars() } }
    }
}

impl<'a> Iterator for StringRewrite<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.chars.next()?;
        match c {
            '\\' => {}
            _ => return Some(c),
        };
        match self.chars.next()? {
            'n' => Some('\n'),
            'r' => Some('\r'),
            't' => Some('\t'),
            '\\' => Some('\\'),
            'u' => {
                let mut s = String::with_capacity(4);
                for _ in 0..4 {
                    s.push(self.chars.next()?);
                }
                Some(char::from_u32(u32::from_str_radix(&s, 16).unwrap()).unwrap())
            }
            c => Some(c),
        }
    }
}
