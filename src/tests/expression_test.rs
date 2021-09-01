use crate::entity::Expression;

#[test]
fn expression_test() {
    let expression = Expression {
        lhs: false,
        rhs: false,
    };
    assert_eq!(expression.lhs, false);
    assert_eq!(expression.rhs, false);
    let expression = expression.lhs(true);
    assert_eq!(expression.lhs, true);
    let expression = expression.rhs(true);
    assert_eq!(expression.rhs, true);
}

#[test]
fn create_expression_test() {
    let expression = Expression::new(false, false);
    assert_eq!(expression.lhs, false);
    assert_eq!(expression.rhs, false);
    let expression = expression.lhs(true);
    assert_eq!(expression.lhs, true);
    let expression = expression.rhs(true);
    assert_eq!(expression.rhs, true);
}
