// Represents a logical operator
#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
}

// Represents either a single source proposition, or another Expression called a subexpression
#[derive(Debug)]
enum PropositionToken {
    Proposition(char),
    Subexpression(Expression),
}

// Represents a proposition or a subexpression, and whether it is negated or not
#[derive(Debug)]
struct ExpressionElement {
    element: PropositionToken,
    negation: bool,
}

impl ExpressionElement {
    fn new(element: PropositionToken, negation: bool) -> Self {
        Self {
            element,
            negation,
        }
    }
}

// Represents a logical expression, which is a recursive tree of propositions/subexpressions and operators
#[derive(Debug)]
pub struct Expression {
    propositions: Vec<ExpressionElement>,
    operators: Vec<Operator>,
}

impl Expression {
    fn new(propositions: Vec<ExpressionElement>, operators: Vec<Operator>) -> Self {
        Self {
            propositions,
            operators,
        }
    }

    // Recursively parses an Expression from a string
    pub fn from(string: &str) -> Expression {
        let mut propositions: Vec<ExpressionElement> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new();

        let mut input_chars = string.char_indices();
        let mut is_negated = false;

        use PropositionToken::*;

        while let Some((i, c)) = input_chars.next() {
            // For each char in the expression
            match c {
                // If the proposition character is within the allowed values (based on the assignment instructions)
                'A'..='D' | 'a'..='d' => {
                    propositions.push(ExpressionElement::new(Proposition(c), is_negated));
                    is_negated = false;
                }
                // If a subexpression is encountered
                '(' => {
                    // Get the current subexpression and recursively parse it
                    let subexpression = get_subexpression(&string[i..]);
                    propositions.push(ExpressionElement::new(Subexpression(Self::from(&subexpression)), is_negated));

                    // Skip the subexpression for its parent's parsing
                    input_chars.nth(subexpression.len());

                    is_negated = false;
                }
                // Queue a negation to add to the next ExpressionToken
                '!' | '/' => {
                    is_negated = true;
                }
                '&' | '*' => {
                    operators.push(Operator::And);
                }
                '|' | '+' => {
                    operators.push(Operator::Or);
                }
                // TODO: Add input validation for garbage chars
                _ => (),
            }
        }

        if propositions.len() != operators.len() + 1 {
            panic!("Mismatched proposition/operator count in expression");
        }

        Self {
            propositions,
            operators,
        }
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
    fn test_expression_nonrecursive_parse() {
        let expression = Expression::from("A & B");
        assert_eq!(expression.propositions.len(), 2);
        assert_eq!(expression.operators.len(), 1);
        assert_eq!(expression.operators[0], Operator::And);

        let mut proposition_num = 0;

        for proposition in &expression.propositions {
            match proposition.element {
                PropositionToken::Proposition(p) => {
                    match proposition_num {
                        0 => assert_eq!(p, 'A'),
                        1 => assert_eq!(p, 'B'),
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
    fn test_expression_recursive_parse_1() {
        let expression = Expression::from("(A & B & !C) | (A & B | (!C | A)) & (A | B)");
        assert_eq!(include_str!("test_files/expression_1.tree").trim_end(), format!("{:#?}", expression));
    }

    #[test]
    fn test_expression_recursive_parse_2() {
        let expression = Expression::from("!(A & B) | ((A | !C | !D) & A) & B & C");
        assert_eq!(include_str!("test_files/expression_2.tree").trim_end(), format!("{:#?}", expression));
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
