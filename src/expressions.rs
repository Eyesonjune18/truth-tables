use std::collections::HashMap;

// Represents a logical expression, which is a recursive tree of propositions/subexpressions and operators
// TODO: Explain propositions field
#[derive(Debug)]
pub struct Expression {
    elements: Vec<ExpressionElement>,
    operators: Vec<Operator>,
    propositions: HashMap<PropositionIdentifier, Option<bool>>,
}

// Represents a proposition or a subexpression, and whether it is negated or not
#[derive(Debug)]
struct ExpressionElement {
    element: PropositionToken,
    negation: bool,
}

// Represents a logical operator
#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
}

// Represents either a single source proposition, or another Expression called a subexpression
#[derive(Debug)]
enum PropositionToken {
    Proposition(PropositionIdentifier),
    Subexpression(Expression),
}

// Represents one of the allowed root proposition letters ("identifiers")
#[derive(Eq, PartialEq, Hash, Debug)]
enum PropositionIdentifier {
    A,
    B,
    C,
    D,
}

impl ExpressionElement {
    fn new(element: PropositionToken, negation: bool) -> Self {
        Self { element, negation }
    }

    // Converts a char to a proposition ExpressionElement
    // TODO: Add range checking here? Probably not necessary
    fn from_proposition(proposition_letter: char, negation: bool) -> Self {
        Self {
            element: PropositionToken::Proposition(PropositionIdentifier::from(proposition_letter)),
            negation,
        }
    }
}

impl PropositionIdentifier {
    // Returns the masked value of the proposition for a given permutation of propositions
    // TODO: How to handle missing propositions?
    fn mask(&self, permutation: u8) -> bool {
        match self {
            Self::A => permutation & 0b0001 != 0,
            Self::B => permutation & 0b0010 != 0,
            Self::C => permutation & 0b0100 != 0,
            Self::D => permutation & 0b1000 != 0,
        }
    }

    // Converts a char to a PropositionIdentifier
    fn from(c: char) -> Self {
        match c {
            'a' | 'A' => Self::A,
            'b' | 'B' => Self::B,
            'c' | 'C' => Self::C,
            'd' | 'D' => Self::D,
            _ => unreachable!("[INTERNAL ERROR] Invalid proposition character '{}'", c),
        }
    }
}

impl Expression {
    fn new(elements: Vec<ExpressionElement>, operators: Vec<Operator>, propositions: HashMap<PropositionIdentifier, Option<bool>>) -> Self {
        Self {
            elements,
            operators,
            propositions,
        }
    }

    // Recursively parses an Expression from a string
    pub fn parse(expression_string: &str) -> Expression {
        let mut elements: Vec<ExpressionElement> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new();

        let mut input_chars = expression_string.char_indices();
        let mut is_negated = false;

        use PropositionToken::*;

        while let Some((i, c)) = input_chars.next() {
            // For each char in the expression
            match c {
                // If the proposition character is within the allowed values (based on the assignment instructions)
                'A'..='D' | 'a'..='d' => {
                    elements.push(ExpressionElement::from_proposition(c, is_negated));
                    is_negated = false;
                }
                // If a subexpression is encountered
                '(' => {
                    // Get the current subexpression and recursively parse it
                    let subexpression = get_subexpression(&expression_string[i..]);
                    elements.push(ExpressionElement::new(
                        Subexpression(Self::parse(&subexpression)),
                        is_negated,
                    ));

                    // Skip the subexpression for its parent's parsing
                    input_chars.nth(subexpression.len());

                    is_negated = false;
                }
                // If a subexpression is not properly skipped
                ')' => panic!("Unmatched ')' in expression"),
                // Queue a negation to add to the next ExpressionToken
                '!' | '/' => is_negated = true,
                '&' | '*' => operators.push(Operator::And),
                '|' | '+' => operators.push(Operator::Or),
                // Ignore whitespace
                ' ' | '\n' => (),
                // Panic on unknown characters
                _ => panic!("Invalid character '{}' in expression", c),
            }
        }

        // Ensure the correct number of elements and operators
        if elements.len() != operators.len() + 1 {
            panic!("Mismatched proposition/operator count in expression");
        }

        Self::new(elements, operators, get_unique_propositions_unvalued(expression_string))
    }

    // Evaluates a single permutation of propositions
    pub fn evaluate_single(&self) -> bool {
        todo!()
    }
}

// Return the substring between the first pair of parentheses, excluding the parentheses themselves
fn get_subexpression(expression: &str) -> String {
    // If the first character is not a '(', panic with an error message
    if expression.chars().next().unwrap() != '(' {
        unreachable!("[INTERNAL ERROR] Subexpression must start with '('");
    }

    let mut subexpression = String::new();
    let mut depth = 1;

    for c in expression.chars().skip(1) {
        // Adjust the nesting depth to determine when to stop
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => (),
        }

        // Create the substring from the text inside the outerost parentheses
        // Stop as soon as the corresponding close parentheses have been found
        if depth > 0 {
            subexpression.push(c);
        } else {
            break;
        }
    }

    subexpression
}

// Returns a list of all unique 
fn get_unique_propositions_unvalued(expression: &str) -> HashMap<PropositionIdentifier, Option<bool>> {
    let mut propositions: HashMap<PropositionIdentifier, Option<bool>> = HashMap::new();

    for c in expression.chars() {
        match c {
            'A'..='D' | 'a'..='d' => {
                propositions.insert(PropositionIdentifier::from(c), None);
            },
            _ => (),
        }
    }

    propositions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_nonrecursive_parse() {
        let expression = Expression::parse("A & B");
        assert_eq!(expression.elements.len(), 2);
        assert_eq!(expression.operators.len(), 1);
        assert_eq!(expression.operators[0], Operator::And);

        let mut proposition_num = 0;

        for proposition in &expression.elements {
            match &proposition.element {
                PropositionToken::Proposition(p) => {
                    match proposition_num {
                        0 => assert_eq!(p, &PropositionIdentifier::A),
                        1 => assert_eq!(p, &PropositionIdentifier::B),
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
    fn test_get_subexpression_nested_single() {
        let expression = "((A | B) & C)";
        assert_eq!(get_subexpression(expression), "(A | B) & C");
    }

    #[test]
    fn test_get_subexpression_nested_multi() {
        let expression = "((A | B) & C) & (D & C & A)";
        assert_eq!(get_subexpression(expression), "(A | B) & C");
    }

    #[test]
    fn test_get_subexpression() {
        let expression = "(A | B & C)";
        assert_eq!(get_subexpression(expression), "A | B & C");
    }
}
