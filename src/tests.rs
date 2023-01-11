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

    if let ExpressionToken::Proposition(p) = &expression.propositions[0] {
        assert_eq!(p.letter, 'A');
    } else {
        assert!(false);
    }

    if let ExpressionToken::Proposition(p) = &expression.propositions[1] {
        assert_eq!(p.letter, 'B');
    } else {
        assert!(false);
    }
}

#[test]
fn test_subexpression_nested_single() {
    let expression = "((A | B) & C)";
    assert_eq!(Expression::get_subexpression(expression), "(A | B) & C");
}

#[test]
fn test_subexpression_nested_multi() {
    let expression = "((A | B) & C) & (D & C & A)";
    assert_eq!(Expression::get_subexpression(expression), "(A | B) & C");
}

#[test]
fn test_subexpression() {
    let expression = "(A | B & C)";
    assert_eq!(Expression::get_subexpression(expression), "A | B & C");
}
