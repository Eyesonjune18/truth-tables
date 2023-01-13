mod expressions;
mod propositions;
mod truth_table;

pub use expressions::Expression;
pub use propositions::PropositionIdentifier;
pub use propositions::PropositionTable;

use crate::truth_table::TruthTable;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} [-e | --expression] [-t | --truth-table] <input>", args[0]);
        println!("Note: The flag you choose determines the input type, not the output type");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "-e" | "--expression" => {
            let mut expression = Expression::parse(&args[2], true);
            let table = TruthTable::from_expression(&mut expression);

            table.print();
        }
        "-t" | "--truth-table" => todo!(),
        _ => panic!("Illegal input formatting based on given flag"),
    }
}
