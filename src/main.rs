use std::env;
use std::fs;

use marclang::lexer::Lexer;
use marclang::parser::Parser;
use marclang::visitor::Visitor;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() != 1 {
        panic!("Invalid number of arguments. You should only provide the path to the file to run.");
    }

    let code = fs::read_to_string(&args[0]).expect("Unable to read file.");

    let lexer = Lexer::new(code);
    let parser = Parser::new(lexer);
    let mut visitor = Visitor::new(parser);

    let output = visitor.run();
    print!("{}", output);
}
