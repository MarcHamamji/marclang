use self::ast::AST;
use crate::lexer::token::Token;

pub mod ast;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> AST {
        let ast = AST::new();

        ast
    }
}
