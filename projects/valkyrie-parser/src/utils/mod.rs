use std::sync::LazyLock;

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
    let mut count = 0;
    for char in rest.chars() {
        match char {
            '~' => count += 1,
            _ => break,
        }
    }
    let mut length = 0;
    match count {
        // not comment
        0 => Err("Not a comment")?,
        // line comment
        1 => {
            length += count;
            for char in rest.chars().skip(count) {
                match char {
                    '\n' => break,
                    _ => length += char.len_utf8(),
                }
            }
        }
        2 => Err("Document comment is not caught")?,
        // block comment
        _ => {
            length += count;
            let mut consecutive = 0;
            for char in rest.chars().skip(count) {
                match char {
                    '~' => {
                        consecutive += 1;
                        if consecutive == count {
                            length += count;
                            break;
                        }
                    }
                    _ => {
                        consecutive = 0;
                        length += char.len_utf8();
                    }
                }
            }
        }
    }
    Ok((&rest[length..], length))
}

static BINARY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^((?x) 
    (\bas\b)[*!?]?
#    | (\bnot\b)?\s+\bin\b | [!¬]?(\bin\b)
#    | (\bis)\b\s+(\bnot\b)? | [!¬]?(\bis\b)
    | [+-]{1,2}=?
    | [⋅⋆∙∗*×⨯⨉⊗⨂/÷]=?
    | [!¬]?([∋∍∊∈∉∌]|<:|:>|[⋢⋣⊑⊒])
    | \^=?
    | ([|&]{1,2}|[∧⊼⩟∨⊽⊻])=?
    | [<>]{1,3}=?
    | [⋃⋂]
    | ={1,3} | ≠ | ≢
)",
    )
    .unwrap()
});

pub fn is_binary(rest: &str) -> Result<(&str, usize), &'static str> {
    match BINARY.find(rest).map(|s| s.as_str()) {
        Some(s) => Ok((s, s.len())),
        None => Err("Not a binary"),
    }
}

static PREFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^((?x) 
    [+-]
    | [!¬]
    | [∂]
    | [√∛∜]
    | [⅟½↉⅓⅔¼¾⅕⅖⅗⅘⅙⅚⅐⅛⅜⅝⅞]
)",
    )
    .unwrap()
});

pub fn is_prefix(rest: &str) -> Result<(&str, usize), &'static str> {
    match PREFIX.find(rest).map(|s| s.as_str()) {
        Some(s) => Ok((s, s.len())),
        None => Err("Not a prefix"),
    }
}

static SUFFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^((?x) 
     [?!]
    | [%‰‱]
    | [℃℉]
    | [°′″‴⁗]
)",
    )
    .unwrap()
});

pub fn is_suffix(rest: &str) -> Result<(&str, usize), &'static str> {
    match SUFFIX.find(rest).map(|s| s.as_str()) {
        Some(s) => Ok((s, s.len())),
        None => Err("Not a suffix"),
    }
}
