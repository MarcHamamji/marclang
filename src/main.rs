use lexer::Lexer;
use std::env;
use std::fs;

mod lexer;

fn main() {
    let filepath = env::args().nth(1).expect("Please specify a file to run");
    let file_contents = fs::read_to_string(filepath).expect("Could not read file");
    let mut lexer = Lexer::new(&file_contents);
    let tokens = lexer.lex();
    print!("{:#?}", tokens);
}
