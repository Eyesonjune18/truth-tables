use std::collections::HashMap;
use std::ops::Range;

use crate::Expression;
use crate::PropositionIdentifier;

// Represents a truth table for a given expression
// Proposition value permutations are encoded in u8s
pub struct TruthTable {
    #[allow(dead_code)]
    // TODO: Not sure whether this is needed or not, keeping for now
    expression: Expression,
    propositions: Vec<PropositionIdentifier>,
    values_and_results: HashMap<u8, bool>,
}

impl TruthTable {
    fn new(
        expression: Expression,
        propositions: Vec<PropositionIdentifier>,
        values_and_results: HashMap<u8, bool>,
    ) -> Self {
        Self {
            expression,
            propositions,
            values_and_results,
        }
    }

    // Creates a new truth table for a given expression
    pub fn from_expression(mut expression: Expression) -> Self {
        let proposition_count = expression.proposition_count();

        let mut propositions = Vec::new();
        let mut values_and_results = HashMap::new();

        // Get all propositions in the expression
        // It is assumed that the propositions are named A, B, C, and D, and will never be out of order
        // TODO: Get this from the expression?
        for i in 0..proposition_count {
            propositions.push(PropositionIdentifier::from_int(i));
        }

        // Generate all possible permutations of the propositions
        for permutation in get_bit_permutations(proposition_count) {
            values_and_results.insert(permutation, expression.evaluate_permutation(permutation));
        }

        Self::new(expression, propositions, values_and_results)
    }

    // TODO: Fix code, this is auto-generated
    pub fn print(&self) {
        // Print the header
        for proposition in &self.propositions {
            print!("{} ", proposition.to_char());
        }

        println!("Result");

        // Print the values and results
        for (permutation, result) in &self.values_and_results {
            for i in 0..self.propositions.len() {
                print!(
                    "{} ",
                    if permutation & (1 << i) != 0 {
                        "T"
                    } else {
                        "F"
                    }
                );
            }

            println!("{}", if *result { "T" } else { "F" });
        }

        println!();
    }
}

// Gets a range of numbers with all possible permutations of a given number of bits
fn get_bit_permutations(bits: u8) -> Range<u8> {
    0..(1 << bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_permutations() {
        assert_eq!(get_bit_permutations(0).collect::<Vec<u8>>(), vec![0]);
        assert_eq!(get_bit_permutations(1).collect::<Vec<u8>>(), vec![0, 1]);
        assert_eq!(
            get_bit_permutations(2).collect::<Vec<u8>>(),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            get_bit_permutations(3).collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7]
        );
    }
}
