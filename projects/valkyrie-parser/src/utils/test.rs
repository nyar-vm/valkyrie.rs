use super::*;

#[test]
fn test_comment() -> Result<(), &'static str> {
    assert_eq!(is_comment("~ 行注释\n 其他其他")?.1, 11);
    assert_eq!(is_comment("~ line comment\n what what")?.1, 14);
    assert_eq!(is_comment("~~~ block comment ~~~ what")?.1, 21);
    assert_eq!(is_comment("~~~~~ 块注释 ~~~~~ 其他其他")?.1, 21);
    Ok(())
}


#[test]
fn test_prefix() -> Result<(), &'static str> {
    assert_eq!(is_prefix("+ rhs")?.1, 1);
    assert_eq!(is_prefix("++ rhs")?.1, 1);
    assert_eq!(is_prefix("++= rhs")?.1, 1);
    Ok(())
}

#[test]
fn test_infix() -> Result<(), &'static str> {
    assert_eq!(is_binary("+ rhs")?.1, 1);
    assert_eq!(is_binary("++ rhs")?.1, 2);
    assert_eq!(is_binary("++= rhs")?.1, 3);
    Ok(())
}