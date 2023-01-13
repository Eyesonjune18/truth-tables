use crate::PropositionIdentifier;
use crate::PropositionTable;

// Represents a logical expression, which is a recursive tree of propositions/subexpressions and operators
// Also includes a table of all proposition letters used in the expression, and their respective values
#[derive(Debug)]
pub struct Expression {
    elements: Vec<ExpressionElement>,
    operators: Vec<Operator>,
    propositions: PropositionTable,
}

// Represents a proposition or a subexpression, and whether it is negated or not
#[derive(Debug)]
struct ExpressionElement {
    token: ExpressionElementToken,
    negation: bool,
}

// Represents either a single source proposition, or another Expression called a subexpression
#[derive(Debug)]
enum ExpressionElementToken {
    Proposition(PropositionIdentifier),
    Subexpression(Expression),
}

// Represents a logical operator
#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
}

impl ExpressionElement {
    fn new(element: ExpressionElementToken, negation: bool) -> Self {
        Self {
            token: element,
            negation,
        }
    }

    // Converts a char to a proposition ExpressionElement
    // TODO: Add range checking here? Probably not necessary
    fn from_proposition(proposition_letter: char, negation: bool) -> Self {
        Self::new(
            ExpressionElementToken::Proposition(PropositionIdentifier::from_char(proposition_letter)),
            negation,
        )
    }
}

impl Expression {
    fn new(
        elements: Vec<ExpressionElement>,
        operators: Vec<Operator>,
        propositions: PropositionTable,
    ) -> Self {
        Self {
            elements,
            operators,
            propositions,
        }
    }

    // Recursively parses an Expression from a string
    pub fn parse(expression_string: &str, validate_propositions: bool) -> Expression {
        let mut elements: Vec<ExpressionElement> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new();
        let propositions = PropositionTable::from_str(expression_string);

        // Make sure that the expression does not skip propositions such as in (A, B, D) or (C, D)
        if validate_propositions && !propositions.validate() {
            panic!("Expression does not contain purely consecutive proposition identifiers");
        }

        let mut input_chars = expression_string.char_indices();
        let mut is_negated = false;

        use ExpressionElementToken::*;

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
                        Subexpression(Self::parse(&subexpression, false)),
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

        Self::new(elements, operators, propositions)
    }

    // Recursively sets the values of all propositions in the expression and its subexpressions
    pub fn set_values(&mut self, permutation: u8) {
        // Set the proposition values in the current expression
        self.propositions.set_all(permutation);
        
        use ExpressionElementToken::*;

        // Set the proposition values in all subexpressions recursively
        for element in &mut self.elements {
            match &mut element.token {
                Subexpression(e) => e.set_values(permutation),
                Proposition(_) => (),
            }
        }
    }

    // Recursively evaluates the expression based on its current table
    // The table must be set before calling this function, or it will cause an error
    fn evaluate(&self) -> bool {
        // Evaluate the first element
        let mut result = self.evaluate_element(&self.elements[0]);

        // Evaluate the remaining elements and operators
        for (i, operator) in self.operators.iter().enumerate() {
            let element = &self.elements[i + 1];

            match operator {
                Operator::And => result &= self.evaluate_element(element),
                Operator::Or => result |= self.evaluate_element(element),
            }
        }

        result
    }

    // Evaluates an ExpressionElement, which can be a proposition or a subexpression
    // Subexpressions are evaluated recursively
    fn evaluate_element(&self, element: &ExpressionElement) -> bool {
        use ExpressionElementToken::*;

        let mut result = match &element.token {
            Proposition(p) => self.propositions.get_value(p).expect(
                "[INTERNAL ERROR] Expression proposition values were not set before evaluation",
            ),
            Subexpression(s) => s.evaluate(),
        };

        if element.negation {
            result = !result;
        }

        result
    }

    // Returns the number of propositions in the expression
    pub fn proposition_count(&self) -> u8 {
        self.propositions.count()
    }

    // Evaluates a single permutation of propositions
    pub fn evaluate_permutation(&mut self, permutation: u8) -> bool {
        self.set_values(permutation);
        self.evaluate()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nonrecursive() {
        let expression = Expression::parse("A & B", true);
        assert_eq!(expression.elements.len(), 2);
        assert_eq!(expression.operators.len(), 1);
        assert_eq!(expression.operators[0], Operator::And);

        let mut proposition_num = 0;

        for proposition in &expression.elements {
            match &proposition.token {
                ExpressionElementToken::Proposition(p) => {
                    match proposition_num {
                        0 => assert_eq!(p, &PropositionIdentifier::A),
                        1 => assert_eq!(p, &PropositionIdentifier::B),
                        _ => assert!(false),
                    }

                    assert_eq!(proposition.negation, false);
                }
                ExpressionElementToken::Subexpression(_) => {
                    assert!(false);
                }
            }

            proposition_num += 1;
        }
    }

    #[test]
    fn test_evaluate_nonrecursive() {
        let mut expression = Expression::parse("A & B", true);

        expression.set_values(0b0000);
        assert!(!expression.evaluate());

        expression.set_values(0b0010);
        assert!(!expression.evaluate());

        expression.set_values(0b0001);
        assert!(!expression.evaluate());

        expression.set_values(0b0011);
        assert!(expression.evaluate());

        expression = Expression::parse("!A & !B", true);

        expression.set_values(0b0000);
        assert!(expression.evaluate());

        expression.set_values(0b0010);
        assert!(!expression.evaluate());

        expression.set_values(0b0001);
        assert!(!expression.evaluate());

        expression.set_values(0b0011);
        assert!(!expression.evaluate());
    }

    #[test]
    fn test_evaluate_recursive() {
        let mut expression = Expression::parse("(A & B) | (C & D)", true);

        for i in 0..=15 {
            expression.set_values(i);
            assert_eq!(
                expression.evaluate(),
                i == 0b1100
                    || i == 0b1110
                    || i == 0b1101
                    || i == 0b0011
                    || i == 0b1011
                    || i == 0b0111
                    || i == 0b1111
            );
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
