mod expressions;
mod propositions;
mod truth_table;

pub use expressions::Expression;
pub use propositions::PropositionIdentifier;
pub use propositions::PropositionTable;

use crate::truth_table::TruthTable;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <expression>", args[0]);
    }

    let mut expression = Expression::parse(&args[1], true);
    let table = TruthTable::from_expression(&mut expression);

    table.print();
}
