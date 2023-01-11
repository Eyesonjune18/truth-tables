#![cfg(test)]

use super::*;

#[test]
fn test_expression_nonrecursive_parse() {
    let expression = Expression::from("A & B");
    assert_eq!(expression.propositions.len(), 2);
    assert_eq!(expression.operators.len(), 1);
    assert_eq!(expression.operators[0], Operator::And);

    let mut proposition_num = 0;

    for proposition in &expression.propositions {
        match proposition.element {
            PropositionToken::Proposition(p) => {
                match proposition_num {
                    0 => assert_eq!(p, 'A'),
                    1 => assert_eq!(p, 'B'),
                    _ => assert!(false),
                }

                assert_eq!(proposition.negation, false);
            }
            PropositionToken::Subexpression(_) => {
                assert!(false);
            }
        }

        proposition_num += 1;
    }
}

#[test]
fn test_expression_recursive_parse_1() {
    let expression = Expression::from("(A & B & !C) | (A & B | (!C | A)) & (A | B)");
    assert_eq!(include_str!("test_files/expression_1.tree").trim_end(), format!("{:#?}", expression));
}

#[test]
fn test_expression_recursive_parse_2() {
    let expression = Expression::from("!(A & B) | ((A | !C | !D) & A) & B & C");
    assert_eq!(include_str!("test_files/expression_2.tree").trim_end(), format!("{:#?}", expression));
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
