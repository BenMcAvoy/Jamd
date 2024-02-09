mod ast;

use ast::parser::Parser;
use ast::Ast;

fn main() {
    let input = "(7 - 2) * 9";

    let parser = Parser::from_input(input);
    let mut ast = Ast::default();

    parser.for_each(|statement| {
        ast.add_statement(statement);
    });

    ast.visualize();

    println!("\nEvaluating...");
    let mut evaluator = ast::evaluator::Evaluator::default();

    ast.visit(&mut evaluator);

    if let Some(result) = evaluator.last_value {
        println!("Result: {result}");
    }
}
