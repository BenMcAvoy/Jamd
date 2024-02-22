mod ast;
mod diagnostics;
mod text;

use std::cell::RefCell;
use std::rc::Rc;

use ast::parser::Parser;
use ast::Ast;

use crate::ast::evaluator::Evaluator;
use crate::diagnostics::BagCell;

fn main() {
    // Check if user supplied a valid file
    let args: Vec<String> = std::env::args().collect();

    let input = args.get(1).map_or_else(
        || String::from("7 - (2 * 9)\n2 - 5"),
        |file| std::fs::read_to_string(file).expect("Failed to read file"),
    );

    let diagnostics_bag: BagCell = Rc::new(RefCell::new(diagnostics::Bag::default()));

    let parser = Parser::from_input(&input, diagnostics_bag);

    let mut ast = Ast::default();

    parser.for_each(|statement| {
        ast.add_statement(statement);
    });

    ast.visualize();

    println!("\nEvaluating...");
    let mut evaluator = Evaluator::default();
    ast.visit(&mut evaluator);

    // Print values nicer:
    let values = evaluator
        .values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ");

    println!("\nStatement return values: {values}");
}
