use super::*;

const INDEX: &str = r#"
a.1
a
    .1

a[1]

a   [
    2
    ]

a
[3,4]

a
[5]
[6]
"#;

#[test]
fn debug_index() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(INDEX);
    ast.save("tests/expression/debug_index.clj")
}

const APPLY: &str = r#"
a({}) {}
"#;

#[test]
fn debug_apply2() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(APPLY);
    ast.save("tests/expression/slice.vk")
}
use super::*;

#[test]
fn debug_slice() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(SLICE);
    ast.save("tests/test_expr/debug_slice.yaml")
}

const INDEX: &str = r#"
a[1]

a   [
    2
    ]

a
[3,4]

a
[5]
[6]
"#;

#[test]
fn debug_index() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(INDEX);
    ast.save("tests/test_expr/debug_index.yaml")
}
use super::*;

const INFIX: &str = r#"
1 + 1;
2 ++ 2;
3 + + 3;
4 +++ 4
4 + ++ 4;
5 ++ + 5;
6 ++++ 6;
7 + +++ 7;
8 ++ ++ 8;
9 +++ + 9;
"#;

#[test]
fn debug_infix() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(INFIX);
    ast.save("tests/test_expr/debug_infix.yaml")
}

const INFIX2: &str = r#"
1 + 2 * 3;
(1+2) * 3;
1 + 2 * 3 + 4 * 5 * 6;
"#;

#[test]
fn debug_infix2() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(INFIX2);
    ast.save("tests/test_expr/debug_infix2.yaml")
}

const MIX_INFIX: &str = r#"
1 > 2 > 3;
+1+2*3^-4!!;
"#;

#[test]
fn debug_mix_infix() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(MIX_INFIX);
    ast.save("tests/test_expr/debug_mix_infix.yaml")
}
use super::*;

const UNARY: &str = r#"
+1;
2!;
+3!;
-+4!!;
"#;

#[test]
fn debug_unary() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(UNARY);
    ast.save("tests/test_expr/debug_unary.yaml")
}
