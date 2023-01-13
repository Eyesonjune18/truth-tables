use std::collections::BTreeMap;
use std::ops::Range;

use crate::Expression;
use crate::PropositionIdentifier;

// Represents a truth table for a given expression
// Proposition value permutations are encoded in u8s
pub struct TruthTable {
    propositions: Vec<PropositionIdentifier>,
    values_and_results: BTreeMap<u8, bool>,
}

impl Default for TruthTable {
    fn default() -> Self {
        Self {
            propositions: Vec::new(),
            values_and_results: BTreeMap::new(),
        }
    }
}

impl TruthTable {
    fn new(
        propositions: Vec<PropositionIdentifier>,
        values_and_results: BTreeMap<u8, bool>,
    ) -> Self {
        Self {
            propositions,
            values_and_results,
        }
    }

    // Creates a new truth table for a given expression
    pub fn from_expression(expression: &mut Expression) -> Self {
        let proposition_count = expression.proposition_count();

        let mut propositions = Vec::new();
        let mut values_and_results = BTreeMap::new();

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

        Self::new(propositions, values_and_results)
    }

    // Parses a user-inputted set of rows into a truth table
    pub fn parse_rows(rows: &str) -> Self {
        todo!()
    }

    // Parses a user-inputted string into an Expression, then into a truth table
    pub fn parse_expression_str(expression: &str) -> Self {
        let mut expression = Expression::parse(expression, true);
        Self::from_expression(&mut expression)
    }

    pub fn print(&self) {
        let mut num_dividers = 8;

        // Print the header
        for proposition in &self.propositions {
            print!("{} ", proposition.to_char());
            num_dividers += 2;
        }

        println!("│ Result");

        // Print the dividers
        for i in 0..num_dividers {
            if i == num_dividers - 8 {
                print!("┼");
            } else {
                print!("─");
            }
        }

        println!();

        // Print the values and results
        for (permutation, result) in &self.values_and_results {
            for i in 0..self.propositions.len() {
                // Mask out the appropriate bit and turn it into a 0 or a 1
                let proposition_bit = (permutation & (1 << i)) >> i;
                print!("{} ", proposition_bit);
            }

            println!("│      {}", if *result { "T" } else { "F" });
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
