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
                    _ => offset += 1,
                }
            }
            Ok((&rest[offset..], offset))
        }
        _ => {
            if rest.chars().nth(length) == Some('\n') {
                Ok((&rest[length + 1..], length))
            } else {
                Err("Not a comment")
            }
        }
    }
}