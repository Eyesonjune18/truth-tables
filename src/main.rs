#![allow(dead_code)]

pub mod expressions;

pub use expressions::Expression;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <expression>", args[0]);
    }

    Expression::parse(&args[1]);
}
