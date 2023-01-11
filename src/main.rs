#![allow(dead_code)]

pub mod tests;

// Represents either a single source proposition, or another Expression called a subexpression
#[derive(Debug)]
enum ExpressionToken {
    Proposition(char),
    Subexpression(Expression),
}

// Represents a logical operator
#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
    Not,
}

// Represents a logical expression, which is a list of propositions/subexpressions and operators
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
        let mut is_paired_with_not = false;

        while let Some((i, c)) = input_chars.next() {
            // For each char in the expression
            match c {
                'A'..='D' => {
                    expression.propositions.push(ExpressionToken::Proposition(c));

                    if is_paired_with_not {
                        expression.operators.push(None);
                        is_paired_with_not = false;
                    }
                },
                '(' => {
                    let subexpression = Self::get_subexpression(&string[i..]);

                    expression.propositions.push(ExpressionToken::Subexpression(Self::from(&subexpression)));

                    if is_paired_with_not {
                        expression.operators.push(None);
                        is_paired_with_not = false;
                    }
                },
                '!' => {
                    expression.operators.push(Some(Operator::Not));
                    is_paired_with_not = true;
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
        let mut subexpression = String::new();
        let mut depth = 1;

        for c in expression.chars() {
            // Stop when the corresponding close parentheses have been found
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => (),
            }

            if depth > 0 {
                subexpression.push(c);
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
