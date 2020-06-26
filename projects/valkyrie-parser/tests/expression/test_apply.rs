const APPLY: &str = r#"
a({}) {}
"#;

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

const INFIX2: &str = r#"
1 + 2 * 3;
(1+2) * 3;
1 + 2 * 3 + 4 * 5 * 6;
"#;

const MIX_INFIX: &str = r#"
1 > 2 > 3;
+1+2*3^-4!!;
"#;

const UNARY: &str = r#"
+1;
2!;
+3!;
-+4!!;
"#;
