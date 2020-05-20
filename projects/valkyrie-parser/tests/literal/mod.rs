use super::*;

#[test]
fn lex_number() {
    repl_debug(include_str!("number.vk"), "literal/number_debug.rkt").expect("number");
    repl_debug(include_str!("symbol.vk"), "literal/symbol_debug.rkt").expect("symbol");
    repl_debug(include_str!("multiline.vk"), "literal/multiline_debug.rkt").expect("multiline");
}

#[test]
fn test_table() {
    let mut file = File::create(here().join("table_debug.rkt")).expect("File to create `table_debug.rkt`");
    let apply = parse_repl(include_str!("table.vk"));
    for expr in &apply {
        writeln!(file, "{}", colored_lisp(expr.lispify(), 144).unwrap()).unwrap();
        println!("{}", colored_lisp(expr.lispify(), 144).unwrap());
    }
}
