use std::env;
use std::fs;
use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;

fn main() {
    let filepath = env::args().nth(1).expect("Please specify a file to run");
    let file_contents = fs::read_to_string(filepath).expect("Could not read file");

    let mut lexer = Lexer::new(&file_contents);
    let tokens = lexer.lex();
    print!("\n----------------- Tokens -----------------\n{:#?}", tokens);

    let parser = Parser::new(&tokens);
    let ast = parser.parse();
    print!("\n----------------- AST -----------------\n{:#?}", ast);

}
