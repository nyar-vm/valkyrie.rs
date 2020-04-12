use regex::Regex;
#[cfg(test)]
mod test;

/// Check if the string is a comment
///
/// ```vk
/// ~ line comment
///
/// ~~~~
/// block comment
/// ~~~~
/// ```
pub fn is_comment(rest: &str) -> Result<(&str, usize), &'static str> {
    let mut length = 0;
    for char in rest.chars() {
        match char {
            '~' => length += 1,
            _ => break,
        }
    }
    match length {
        // not comment
        0 => Err("Not a comment"),
        // line comment
        1 => {
            let mut offset = 1;
            for char in rest.chars().skip(1) {
                match char {
                    '\n' => break,
                    _ => offset += char.len_utf8(),
                }
            }
            Ok((&rest[offset..], offset))
        }
        2 => {
            Err("Document comment is not caught")
        }
        // block comment
        _ => {
            let mut offset = length;
            let mut consecutive = 0;
            for char in rest.chars().skip(length) {
                match char {
                    '~' => {
                        consecutive += 1;
                        if consecutive == length {
                            offset += length;
                            break;
                        }
                    }
                    _ => {
                        consecutive = 0;
                        offset += char.len_utf8();
                    }
                }
            }
            Ok((&rest[offset..], offset))
        }
    }
}



pub fn is_binary(rest: &str) -> Result<(&str, usize), &'static str> {
    let pattern = Regex::new(r"(?x) (\bas\b)[*!?]?
    | (\bnot\b)?\s+\bin\b | [!¬]?(\bin\b)
    | (\bis)\b\s+(\bnot\b)? | [!¬]?(\bis\b)
    | [+-]{1,2}=?
    | [⋅⋆∗×⨯⨉⊗⨂/÷]=?
    | [!¬]?([∋∍∊∈∉∌]|<:|:>)
    | \^=?
    | ([|&]{1,2}|[∧⊼⩟∨⊽⊻])=?
    | [<>]{1,3}=?
    | [⋃⋂]
    | ={1,3}
").unwrap();
    match pattern.find(rest).map(|s| s.as_str()) {
        Some(s) => Ok((s, s.len())),
        None => Err("Not a binary"),
    }
}

pub fn is_prefix(rest: &str) -> Result<(&str, usize), &'static str> {
    let pattern = Regex::new(r"(?x) [+-]
    | [!¬]
    | [∂]
    | [√∛∜]
").unwrap();
    match pattern.find(rest).map(|s| s.as_str()) {
        Some(s) => Ok((s, s.len())),
        None => Err("Not a prefix"),
    }
}

pub fn is_suffix(rest: &str) -> Result<(&str, usize), &'static str> {
    let pattern = Regex::new(r"(?x) [?!]
    | [%‰‱]
    | [℃℉]
    | [°′″‴⁗]

").unwrap();
    match pattern.find(rest).map(|s| s.as_str()) {
        Some(s) => Ok((s, s.len())),
        None => Err("Not a suffix"),
    }
}