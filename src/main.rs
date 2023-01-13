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
        println!(
            "Usage: {} [-e | --expression] [-t | --truth-table] <input>",
            args[0]
        );
        println!("Note: The flag you choose determines the input type, not the output type");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "-e" | "--expression" => TruthTable::parse_expression_str(&args[2]).print(),
        "-t" | "--truth-table" => TruthTable::parse_rows(&args[2]).print(),
        _ => panic!("Illegal input formatting based on given flag"),
    }
}
