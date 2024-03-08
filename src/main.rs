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
        || {
            String::from(
                "
        let a = 10
        let b = 20
        let c = a + b
        ",
            )
        },
        |file| std::fs::read_to_string(file).expect("Failed to read file"),
    );

    let diagnostics_bag: BagCell = Rc::new(RefCell::new(diagnostics::Bag::default()));

    let parser = Parser::from_input(&input, Rc::clone(&diagnostics_bag));

    let mut ast = Ast::default();

    parser.for_each(|statement| {
        ast.add_statement(statement);
    });

    ast.visualize();

    let text = text::Source::new(input);
    let diagnostics_binding = diagnostics_bag.borrow();
    if !diagnostics_binding.diagnostics.is_empty() {
        let diagnostics_printer =
            diagnostics::printer::Printer::new(&text, &diagnostics_binding.diagnostics);
        diagnostics_printer.print();
    }

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
