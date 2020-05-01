use lispify::{helpers::colored_lisp, Lispify};
use pex::ParseState;
use pratt::PrattParser;
use valkyrie_parser::expression::{ExpressionResolver, ExpressionStream};

#[test]
fn main() {
    let raw = "12 + global::x";
    let tt = ExpressionStream::parse(ParseState::new(raw)).unwrap();
    let expr = ExpressionResolver.parse(&mut tt.into_iter()).unwrap();
    println!("input: {raw}");
    // println!("output:\n {expr:#?}");
    println!("output:\n {}", colored_lisp(expr.lispify(), 42).unwrap());
}
