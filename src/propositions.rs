use std::collections::HashMap;

// Represents one of the allowed root proposition letters ("identifiers")
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum PropositionIdentifier {
    A,
    B,
    C,
    D,
}

// Stores a table of all the proposition identifiers, and their respective values
#[derive(Debug)]
pub struct PropositionTable {
    propositions: HashMap<PropositionIdentifier, Option<bool>>,
}

impl PropositionIdentifier {
    // Returns the masked value of the proposition for a given permutation of propositions, in 0bABCD format
    pub fn mask(&self, permutation: u8) -> bool {
        match self {
            Self::A => permutation & 0b1000 != 0,
            Self::B => permutation & 0b0100 != 0,
            Self::C => permutation & 0b0010 != 0,
            Self::D => permutation & 0b0001 != 0,
        }
    }

    // Converts a char to a PropositionIdentifier
    pub fn from_char(c: char) -> Self {
        match c {
            'a' | 'A' => Self::A,
            'b' | 'B' => Self::B,
            'c' | 'C' => Self::C,
            'd' | 'D' => Self::D,
            _ => unreachable!("[INTERNAL ERROR] Invalid proposition character '{}'", c),
        }
    }

    // Converts a u8 to a PropositionIdentifier
    pub fn from_int(i: u8) -> Self {
        match i {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            _ => unreachable!("[INTERNAL ERROR] Invalid proposition integer '{}'", i),
        }
    }

    // Converts a PropositionIdentifier to a char
    pub fn to_char(&self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
        }
    }
}

impl PropositionTable {
    fn new(propositions: HashMap<PropositionIdentifier, Option<bool>>) -> Self {
        Self { propositions }
    }

    // Parses a string into a PropositionTable
    pub fn from_expression_str(expression: &str) -> Self {
        let mut propositions: HashMap<PropositionIdentifier, Option<bool>> = HashMap::new();

        for c in expression.chars() {
            match c {
                'A'..='D' | 'a'..='d' => {
                    propositions.insert(PropositionIdentifier::from_char(c), None);
                }
                _ => (),
            }
        }

        Self::new(propositions)
    }

    // Returns the value of a proposition in the table
    pub fn get_value(&self, identifier: &PropositionIdentifier) -> Option<bool> {
        self.propositions.get(identifier).copied().flatten()
    }

    // Sets the true/false values of all the propositions in the table by bitmasking a provided u8 (0b0000DCBA)
    pub fn set_all(&mut self, values: u8) {
        for (proposition, value) in self.propositions.iter_mut() {
            *value = Some(proposition.mask(values));
        }
    }

    // Returns the number of propositions in the table
    pub fn count(&self) -> u8 {
        self.propositions.len() as u8
    }

    // Ensures that there are no skipped identifiers
    // This is a really ugly way to do this and it's not very scalable, but it should do fine for this assignment
    pub fn validate(&self) -> bool {
        use PropositionIdentifier::*;

        match self.propositions.len() {
            1 => self.propositions.contains_key(&A),
            2 => self.propositions.contains_key(&A) && self.propositions.contains_key(&B),
            3 => {
                self.propositions.contains_key(&A)
                    && self.propositions.contains_key(&B)
                    && self.propositions.contains_key(&C)
            }
            4 => {
                self.propositions.contains_key(&A)
                    && self.propositions.contains_key(&B)
                    && self.propositions.contains_key(&C)
                    && self.propositions.contains_key(&D)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_propositions() {
        let mut expression = "A";
        assert!(PropositionTable::from_expression_str(expression).validate());

        expression = "A & B";
        assert!(PropositionTable::from_expression_str(expression).validate());

        expression = "A & B & C";
        assert!(PropositionTable::from_expression_str(expression).validate());

        expression = "A & B & C & D";
        assert!(PropositionTable::from_expression_str(expression).validate());

        expression = "A & C & D";
        assert!(!PropositionTable::from_expression_str(expression).validate());

        expression = "B & C";
        assert!(!PropositionTable::from_expression_str(expression).validate());
    }

    #[test]
    fn test_set_values() {
        let expression = "A & B & C & D";
        let mut table = PropositionTable::from_expression_str(expression);

        use PropositionIdentifier::*;

        table.set_all(0b0000);

        assert_eq!(table.get_value(&A), Some(false));
        assert_eq!(table.get_value(&B), Some(false));
        assert_eq!(table.get_value(&C), Some(false));
        assert_eq!(table.get_value(&D), Some(false));

        table.set_all(0b1111);

        assert_eq!(table.get_value(&A), Some(true));
        assert_eq!(table.get_value(&B), Some(true));
        assert_eq!(table.get_value(&C), Some(true));
        assert_eq!(table.get_value(&D), Some(true));

        table.set_all(0b0101);

        assert_eq!(table.get_value(&A), Some(false));
        assert_eq!(table.get_value(&B), Some(true));
        assert_eq!(table.get_value(&C), Some(false));
        assert_eq!(table.get_value(&D), Some(true));
    }
}
