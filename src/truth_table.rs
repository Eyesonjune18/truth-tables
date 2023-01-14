use std::collections::BTreeMap;

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
    fn from_expression(expression: &mut Expression) -> Self {
        let proposition_count = expression.proposition_count();

        let propositions = get_propositions(proposition_count);
        let mut values_and_results = BTreeMap::new();

        // Generate all possible permutations of the propositions
        for permutation in get_bit_permutations(proposition_count) {
            values_and_results.insert(permutation, expression.evaluate_permutation(permutation));
        }

        Self::new(propositions, values_and_results)
    }

    // Parses a user-inputted set of rows into a truth table
    pub fn parse_rows(rows: &str) -> Self {
        // Split and validate the user-inputted rows
        let rows = rows.split(", ").collect::<Vec<&str>>();

        // Get the propositions based on the number of columns
        let propositions = get_propositions((rows[0].len() - 1) as u8);

        // Parse the rows into a map of permutations and their results
        let values_and_results = rows_to_value_map(rows);

        Self::new(propositions, values_and_results)
    }

    // Converts the truth table into a string representation of the expression
    pub fn to_expression_str(&self) -> String {
        // Get the number of propositions in the truth table
        let proposition_count = self.propositions.len() as u8;

        // Add a conjunction for each permutation that evaluates to true
        let mut expression = String::new();

        for (permutation, result) in &self.values_and_results {
            if *result {
                if !expression.is_empty() {
                    expression.push_str(" | ");
                }

                expression.push_str(encode_conjunction(*permutation, proposition_count).as_str());
            }
        }

        expression
    }

    // Parses a user-inputted string into an Expression, then into a truth table
    pub fn parse_expression_str(expression: &str) -> Self {
        let mut expression = Expression::parse(expression, true);
        Self::from_expression(&mut expression)
    }

    // Formats and prints the truth table
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
            for proposition in &self.propositions {
                let proposition_bit = proposition.mask(*permutation) as u8;
                print!("{} ", proposition_bit);
            }

            println!("│      {}", if *result { "T" } else { "F" });
        }

        println!();
    }
}

// Checks a set of rows against formatting requirements
fn validate_rows(rows: &Vec<&str>) {
    // Make sure all rows contain only '0' and '1'
    for row in rows {
        for c in row.chars() {
            if c != '0' && c != '1' {
                panic!("Invalid character '{}' found in row '{}'", c, row);
            }
        }
    }

    // Make sure all rows are the same length, and that they are within the range of 2 to 5
    let row_size = rows[0].len();

    if row_size < 2 || row_size > 5 {
        panic!("Row size must be between 2 and 5, representing up to four proposition rows and one result row");
    }

    for row in rows {
        if row.len() != row_size {
            panic!("All rows must be the same length");
        }
    }
}

// Gets a list of propositions based on the given count
// It is assumed that the propositions are named A, B, C, and D, and will never be out of order
fn get_propositions(proposition_count: u8) -> Vec<PropositionIdentifier> {
    let mut propositions = Vec::new();
    
    for i in 0..proposition_count {
        propositions.push(PropositionIdentifier::from_int(i));
    }
    
    propositions
}

// Parses a set of string-encoded rows into a map of permutations and their results
fn rows_to_value_map(rows: Vec<&str>) -> BTreeMap<u8, bool> {
    // Ensure the rows are valid before attempting to parse them
    validate_rows(&rows);

    let mut values_and_results = BTreeMap::new();

    for row in rows {
        let permutation = decode_permutation_str(row);
        let result = row.chars().last().unwrap() == '1';

        values_and_results.insert(permutation, result);
    }

    values_and_results
}

// Takes a string-encoded row and decodes it into a value permutation
fn decode_permutation_str(row: &str) -> u8 {
    // Last character is the result, so it is ignored
    let row = &row[0..row.len() - 1];

    // Find the number of propositions
    let proposition_count = row.len();

    // Convert to bits and shift based on amount of skipped propositions (0bA/0bAB/0bABC/0bABCD -> 0b0000ABCD)
    u8::from_str_radix(row, 2).unwrap() << (4 - proposition_count)
}

// Takes a value permutation and encodes it into
fn encode_conjunction(permutation: u8, proposition_count: u8) -> String {
    let mut conjunction = String::from('(');

    for i in 0..proposition_count {
        let proposition = PropositionIdentifier::from_int(i);
        let proposition_value = proposition.mask(permutation);

        if proposition_value {
            if conjunction != "(" && i != proposition_count {
                conjunction.push_str(" & ");
            }

            conjunction.push(proposition.to_char());
        }
    }

    conjunction.push(')');

    conjunction
}

// Gets a range of numbers with all possible permutations of a given number of bits
fn get_bit_permutations(bits: u8) -> Vec<u8> {
    let mut permutations = Vec::new();

    for i in 0u8..(1 << bits) {
        permutations.push(i.reverse_bits() >> 4);
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_permutations() {
        assert_eq!(get_bit_permutations(0), vec![0b0000]);
        assert_eq!(get_bit_permutations(1), vec![0b0000, 0b1000]);
        assert_eq!(get_bit_permutations(2), vec![0b0000, 0b1000, 0b0100, 0b1100]);
        assert_eq!(get_bit_permutations(3), vec![0b0000, 0b1000, 0b0100, 0b1100, 0b0010, 0b1010, 0b0110, 0b1110]);
        assert_eq!(get_bit_permutations(4), vec![0b0000, 0b1000, 0b0100, 0b1100, 0b0010, 0b1010, 0b0110, 0b1110, 0b0001, 0b1001, 0b0101, 0b1101, 0b0011, 0b1011, 0b0111, 0b1111]);
    }

    #[test]
    fn test_decode_permutations() {
        assert_eq!(decode_permutation_str("01"), 0b0000);
        assert_eq!(decode_permutation_str("11"), 0b1000);
        assert_eq!(decode_permutation_str("101"), 0b1000);
        assert_eq!(decode_permutation_str("111"), 0b1100);
        assert_eq!(decode_permutation_str("011"), 0b0100);
        assert_eq!(decode_permutation_str("1001"), 0b1000);
        assert_eq!(decode_permutation_str("1011"), 0b1010);
        assert_eq!(decode_permutation_str("1101"), 0b1100);
        assert_eq!(decode_permutation_str("1111"), 0b1110);
        assert_eq!(decode_permutation_str("10001"), 0b1000);
        assert_eq!(decode_permutation_str("10011"), 0b1001);
        assert_eq!(decode_permutation_str("10101"), 0b1010);
        assert_eq!(decode_permutation_str("10111"), 0b1011);
        assert_eq!(decode_permutation_str("11001"), 0b1100);
        assert_eq!(decode_permutation_str("11011"), 0b1101);
        assert_eq!(decode_permutation_str("11101"), 0b1110);
        assert_eq!(decode_permutation_str("11111"), 0b1111);
    }
}
