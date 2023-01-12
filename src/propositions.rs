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
            Self::A => permutation & 0b0001 != 0,
            Self::B => permutation & 0b0010 != 0,
            Self::C => permutation & 0b0100 != 0,
            Self::D => permutation & 0b1000 != 0,
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
}
