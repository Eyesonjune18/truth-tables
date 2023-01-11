#![allow(dead_code)]

pub mod tests;

// Represents either a single source proposition, or another Expression called a subexpression
#[derive(Debug)]
enum ExpressionToken {
    Proposition(Proposition),
    Subexpression(Expression),
}

// Represents a single source proposition, which can be negated
#[derive(Debug)]
struct Proposition {
    letter: char,
    negation: bool,
}

impl Proposition {
    fn new(letter: char, negation: bool) -> Self {
        Self {
            letter,
            negation,
        }
    }
}

// Represents a logical operator
#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
}

// Represents a logical expression, which is a recursive tree of propositions/subexpressions and operators
#[derive(Debug)]
struct Expression {
    propositions: Vec<ExpressionToken>,
    operators: Vec<Option<Operator>>,
}

impl Expression {
    // Creates an Expression from a string
    fn from(string: &str) -> Expression {
        let mut expression = Expression {
            propositions: Vec::new(),
            operators: Vec::new(),
        };

        let mut input_chars = string.char_indices();
        let mut is_negated = false;

        while let Some((i, c)) = input_chars.next() {
            // For each char in the expression
            match c {
                // If the proposition character is within the allowed values (based on the assignment instructions)
                'A'..='D' => {
                    expression.propositions.push(ExpressionToken::Proposition(Proposition::new(c, is_negated)));

                    // TODO: Functionize this?
                    if is_negated {
                        expression.operators.push(None);
                        is_negated = false;
                    }
                },
                '(' => {
                    // Get the current subexpression and recursively parse it
                    let subexpression = Self::get_subexpression(&string[i..]);
                    expression.propositions.push(ExpressionToken::Subexpression(Self::from(&subexpression)));

                    if is_negated {
                        expression.operators.push(None);
                        is_negated = false;
                    }
                },
                // Queue a negation to add to the next ExpressionToken
                '!' => {
                    is_negated = true;
                },
                '&' => {
                    expression.operators.push(Some(Operator::And));
                },
                '|' => {
                    expression.operators.push(Some(Operator::Or));
                },
                // TODO: Add input validation for garbage chars
                _ => (),
            }
        }

        expression
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
}

fn main() {
    let args:Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <expression>", args[0]);
    }

    let expression = Expression::from(&args[1]);
    dbg!(expression);
}
