#![cfg(test)]
use crate::ExpressionToken;

use super::*;

#[test]
fn test_expression_from() {
    let expression = Expression::from("A&B");
    assert_eq!(expression.propositions.len(), 2);
    assert_eq!(expression.operators.len(), 1);
    assert_eq!(expression.operators[0], Some(Operator::And));
    
    for proposition in &expression.propositions {
        if let ExpressionToken::Subexpression(_) = proposition {
            assert!(false);
        }
    }

    if let ExpressionToken::Proposition(c) = expression.propositions[0] {
        assert_eq!(c, 'A');
    } else {
        assert!(false);
    }

    if let ExpressionToken::Proposition(c) = expression.propositions[1] {
        assert_eq!(c, 'B');
    } else {
        assert!(false);
    }
}
