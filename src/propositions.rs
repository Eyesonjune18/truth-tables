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
    // Returns the masked value of the proposition for a given permutation of propositions
    fn mask(&self, permutation: u8) -> bool {
        match self {
            Self::A => permutation & 0b1000 != 0,
            Self::B => permutation & 0b0100 != 0,
            Self::C => permutation & 0b0010 != 0,
            Self::D => permutation & 0b0001 != 0,
        }
    }

    // Converts a char to a PropositionIdentifier
    pub fn from(c: char) -> Self {
        match c {
            'a' | 'A' => Self::A,
            'b' | 'B' => Self::B,
            'c' | 'C' => Self::C,
            'd' | 'D' => Self::D,
            _ => unreachable!("[INTERNAL ERROR] Invalid proposition character '{}'", c),
        }
    }
}

impl PropositionTable {
    fn new(propositions: HashMap<PropositionIdentifier, Option<bool>>) -> Self {
        Self { propositions }
    }

    // Parses a string into a PropositionTable
    pub fn from(expression: &str) -> Self {
        Self {
            propositions: get_unique_propositions_unvalued(expression),
        }
    }

    // Returns the value of a proposition in the table
    pub fn get(&self, identifier: &PropositionIdentifier) -> Option<bool> {
        self.propositions.get(identifier).copied().flatten()
    }

    // Sets the true/false values of all the propositions in the table by bitmasking a provided u8 (0b0000ABCD)
    pub fn set_all(&mut self, values: u8) {
        for (proposition, value) in self.propositions.iter_mut() {
            *value = Some(proposition.mask(values));
        }
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

// Returns a table of all unique proposition identifiers
// TODO: Rename this and probably move it somewhere else
fn get_unique_propositions_unvalued(
    expression: &str,
) -> HashMap<PropositionIdentifier, Option<bool>> {
    let mut propositions: HashMap<PropositionIdentifier, Option<bool>> = HashMap::new();

    for c in expression.chars() {
        match c {
            'A'..='D' | 'a'..='d' => {
                propositions.insert(PropositionIdentifier::from(c), None);
            }
            _ => (),
        }
    }

    propositions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_propositions() {
        let expression = "A";
        assert!(PropositionTable::from(expression).validate());

        let expression = "A & B";
        assert!(PropositionTable::from(expression).validate());

        let expression = "A & B & C";
        assert!(PropositionTable::from(expression).validate());

        let expression = "A & B & C & D";
        assert!(PropositionTable::from(expression).validate());

        let expression = "A & C & D";
        assert!(!PropositionTable::from(expression).validate());

        let expression = "B & C";
        assert!(!PropositionTable::from(expression).validate());
    }

    #[test]
    fn test_set_values() {
        let expression = "A & B & C & D";
        let mut table = PropositionTable::from(expression);

        use PropositionIdentifier::*;

        table.set_all(0b0000);

        assert_eq!(table.get(&A), Some(false));
        assert_eq!(table.get(&B), Some(false));
        assert_eq!(table.get(&C), Some(false));
        assert_eq!(table.get(&D), Some(false));

        table.set_all(0b1111);

        assert_eq!(table.get(&A), Some(true));
        assert_eq!(table.get(&B), Some(true));
        assert_eq!(table.get(&C), Some(true));
        assert_eq!(table.get(&D), Some(true));

        table.set_all(0b1010);

        assert_eq!(table.get(&A), Some(true));
        assert_eq!(table.get(&B), Some(false));
        assert_eq!(table.get(&C), Some(true));
        assert_eq!(table.get(&D), Some(false));
    }
}
