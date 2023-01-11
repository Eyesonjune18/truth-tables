#![allow(dead_code)]

pub mod expressions;

pub use expressions::Expression;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <expression>", args[0]);
    }

    Expression::parse(&args[1]);
}

fn get_bit_permutations(bits: u8) -> Vec<u8> {
    let mut permutations = Vec::new();

    for i in 0..(1 << bits) {
        permutations.push(i);
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_permutations() {
        assert_eq!(get_bit_permutations(0), vec![0]);
        assert_eq!(get_bit_permutations(1), vec![0, 1]);
        assert_eq!(get_bit_permutations(2), vec![0, 1, 2, 3]);
        assert_eq!(get_bit_permutations(3), vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(
            get_bit_permutations(4),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }
}
