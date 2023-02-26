use std::collections::VecDeque;

use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::lexer::Lexer;

use self::ast::Compound;
use self::ast::FunctionCall;
use self::ast::AST;

pub mod ast;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_tokens: VecDeque<Option<Token>>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Self {
            current_token: lexer.get_next_token().expect("Unexpected empty file"),
            lexer,
            next_tokens: VecDeque::new(),
        }
    }

    pub fn parse<'a>(&'a mut self) -> Compound {
        let mut compound = Compound { list: vec![] };
        while !self.done() {
            compound.list.push(self.parse_statement());
        }
        return compound;
    }

    fn done(&self) -> bool {
        self.lexer.done()
    }

    fn is_current_token_kind(&self, kind: TokenKind) -> bool {
        return self.current_token.is_kind(kind);
    }

    fn eat_strict(&mut self, kind: TokenKind) -> Token {
        self.eat(kind).expect("Unexpected end of file")
    }

    fn eat(&mut self, kind: TokenKind) -> Option<Token> {
        if self.is_current_token_kind(kind) {
            if self.next_tokens.len() == 0 {
                let new_token = self.lexer.get_next_token();

                if new_token.is_none() {
                    return None;
                }

                let old_current_token =
                    std::mem::replace(&mut self.current_token, new_token.expect(""));
                Some(old_current_token)
            } else {
                let old_current_token = std::mem::replace(
                    &mut self.current_token,
                    self.next_tokens.pop_front().expect("").expect(""),
                );
                Some(old_current_token)
            }
        } else {
            panic!(
                "Unexpected token {} at position {}",
                &self.current_token.content, &self.current_token.position
            )
        }
    }

    fn parse_statement<'a>(&mut self) -> AST<'a> {
        let value = match &self.current_token.kind {
            TokenKind::ID => match self.peek_token(1).expect("Unexpected end of file").kind {
                TokenKind::LParenthesis => AST::FunctionCall(self.parse_function_call()),
                _ => {
                    panic!(
                        "Unexpected token {} at position {}",
                        &self.current_token.content, &self.current_token.position
                    );
                }
            },
            _ => {
                panic!(
                    "Unexpected token {} at position {}",
                    &self.current_token.content, &self.current_token.position
                );
            }
        };
        self.eat(TokenKind::Semicolon);
        value
    }

    fn peek_token(&mut self, offset: usize) -> Option<&Token> {
        while self.next_tokens.len() < offset {
            self.next_tokens.push_back(self.lexer.get_next_token());
        }
        return self.next_tokens.back().expect("").as_ref();
    }

    fn parse_function_call(&mut self) -> FunctionCall {
        let function_name_token = self.eat_strict(TokenKind::ID);
        let function_name = function_name_token.content;
        self.eat_strict(TokenKind::LParenthesis);
        let mut arguments = vec![];
        while self.current_token.is_kind(TokenKind::String) {
            let current_token = self.eat_strict(TokenKind::String).content;
            arguments.push(current_token);
        }
        self.eat_strict(TokenKind::RParenthesis);
        let fcall = FunctionCall {
            function_name,
            arguments,
        };
        fcall
    }
}
