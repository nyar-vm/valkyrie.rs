use lispify::{helpers::colored_lisp, Lispify};
use pex::ParseState;
use pratt::PrattParser;
use valkyrie_parser::expression::{ExpressionResolver, ExpressionStream};

#[test]
fn main1() {
    let resolver = ExpressionResolver::default();
    let raw = "(y! * 7)";
    let tt = ExpressionStream::parse(ParseState::new(raw)).unwrap();
    let expr = resolver.resolve(tt).unwrap();
    println!("input: {raw}");
    println!("output:\n {expr:#?}");
    println!("output:\n {}", colored_lisp(expr.lispify(), 42).unwrap());
}

#[test]
fn main() {
    let resolver = ExpressionResolver::default();
    let raw = "5cm + 20℃ + global::x? ^ 3 ^ (4 - 2) + 5 * (y! - 1) * 7 ++ list";
    let tt = ExpressionStream::parse(ParseState::new(raw)).unwrap();
    let expr = resolver.resolve(tt).unwrap();
    println!("input:\n{raw}");
    // println!("output:\n {expr:#?}");
    println!("output:\n{}", colored_lisp(expr.lispify(), 42).unwrap());
}
