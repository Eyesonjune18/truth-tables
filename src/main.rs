#![allow(dead_code)]

mod expressions;
mod propositions;
pub use expressions::Expression;
pub use propositions::PropositionIdentifier;
pub use propositions::PropositionTable;

use std::ops::Range;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <expression>", args[0]);
    }

    let mut expression = Expression::parse(&args[1], true);
    expression.set_values(0b0101);

    dbg!(expression);
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
