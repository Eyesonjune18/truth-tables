#![cfg(test)]
use crate::ExpressionToken;

use super::*;

#[test]
fn test_expression_nonrecursive_parse() {
    let expression = Expression::from("A & B");
    assert_eq!(expression.propositions.len(), 2);
    assert_eq!(expression.operators.len(), 1);
    assert_eq!(expression.operators[0], Operator::And);

    let mut proposition_num = 0;

    for proposition in &expression.propositions {
        match proposition {
            ExpressionToken::Proposition(p) => {
                match proposition_num {
                    0 => {
                        assert_eq!(p.letter, 'A');
                        assert_eq!(p.negation, false);
                    }
                    1 => {
                        assert_eq!(p.letter, 'B');
                        assert_eq!(p.negation, false);
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
            ExpressionToken::Subexpression(_) => {
                assert!(false);
            }
        }

        proposition_num += 1;
    }
}

#[test]
fn test_get_subexpression_nested_single() {
    let expression = "((A | B) & C)";
    assert_eq!(Expression::get_subexpression(expression), "(A | B) & C");
}

#[test]
fn test_get_subexpression_nested_multi() {
    let expression = "((A | B) & C) & (D & C & A)";
    assert_eq!(Expression::get_subexpression(expression), "(A | B) & C");
}

#[test]
fn test_get_subexpression() {
    let expression = "(A | B & C)";
    assert_eq!(Expression::get_subexpression(expression), "A | B & C");
}
