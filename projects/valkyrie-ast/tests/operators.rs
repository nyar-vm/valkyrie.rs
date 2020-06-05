use valkyrie_ast::ValkyrieOperator;
use valkyrie_errors::ValkyrieResult;

#[test]
fn infix() -> ValkyrieResult {
    assert_eq!(ValkyrieOperator::parse_infix("+")?, ValkyrieOperator::Add);
    assert_eq!(ValkyrieOperator::parse_infix("-")?, ValkyrieOperator::Subtract);
    Ok(())
}

#[test]
fn infix_in() -> ValkyrieResult {
    assert_eq!(ValkyrieOperator::parse_infix("in")?, ValkyrieOperator::In(true));
    assert_eq!(ValkyrieOperator::parse_infix(" in   ")?, ValkyrieOperator::In(true));
    //
    assert_eq!(ValkyrieOperator::parse_infix(" ∈   ")?, ValkyrieOperator::In(true));
    assert_eq!(ValkyrieOperator::parse_infix(" !∈   ")?, ValkyrieOperator::In(false));
    //
    assert_eq!(ValkyrieOperator::parse_infix("not in")?, ValkyrieOperator::In(false));
    assert_eq!(ValkyrieOperator::parse_infix("  not      in   ")?, ValkyrieOperator::In(false));
    Ok(())
}
