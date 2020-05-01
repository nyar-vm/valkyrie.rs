use lispify::helpers::colored_lisp;
use pex::ParseState;
use pratt::PrattParser;
use valkyrie_parser::expression::{ExpressionResolver, ExpressionStream};

#[test]
fn main() {
    let tt = ExpressionStream::parse(ParseState::new("1 + 2? ^ 3 ^ 4 + 5 * 6! * 7")).unwrap();
    let expr = ExpressionResolver.parse(&mut tt.into_iter()).unwrap();
    println!("{}", colored_lisp(expr.lispify(), 144).unwrap());
}
