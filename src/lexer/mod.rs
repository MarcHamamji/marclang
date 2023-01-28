use self::token::Token;

pub mod token;

#[derive(Debug)]
pub struct Lexer {
    index: usize,
    chars: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Self {
            index: 0,
            chars: input.chars().collect(),
        }
    }

    fn current_char(&self) -> &char {
        return self.chars.get(self.index).expect("Index out of range");
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn done(&self) -> bool {
        self.index >= self.chars.len()
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.skip_whitespace();
        while !self.done() {
            tokens.push(self.collect_token_and_advance());
            self.skip_whitespace();
        }
        tokens
    }

    fn skip_whitespace(&mut self) {
        while !self.done() {
            let current_char = self.current_char();
            if current_char.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn collect_token_and_advance(&mut self) -> Token {
        let current_char = self.current_char();
        match current_char {
            '(' => {
                let token = Token {
                    kind: token::TokenKind::LPAREN,
                    position: self.index,
                    content: None,
                };
                self.advance();
                token
            }
            ')' => {
                let token = Token {
                    kind: token::TokenKind::RPAREN,
                    position: self.index,
                    content: None,
                };
                self.advance();
                token
            }
            '\'' | '"' => self.collect_string_and_advance(),
            ';' => {
                let token = Token {
                    kind: token::TokenKind::SEMICOLON,
                    position: self.index,
                    content: None,
                };
                self.advance();
                token
            }
            c if c.is_ascii_alphanumeric() => self.collect_id_and_advance(),
            _ => panic!(
                "Unexpected character `{}` at position {}",
                current_char, self.index
            ),
        }
    }

    fn collect_id_and_advance(&mut self) -> Token {
        let initial_index = self.index;
        let mut name = String::from("");
        let mut current_char = self.current_char();
        while current_char.is_ascii_alphanumeric() {
            name.push(*current_char);
            self.advance();
            current_char = self.current_char();
        }
        return Token {
            kind: token::TokenKind::ID,
            content: Some(name),
            position: initial_index,
        };
    }

    fn collect_string_and_advance(&mut self) -> Token {
        let initial_index = self.index;
        let mut string = String::from("");
        let mut current_char = self.current_char();
        if token::is_quote(*current_char) {
            self.advance();
            current_char = self.current_char();
        }
        while !token::is_quote(*current_char) {
            string.push(*current_char);
            self.advance();
            current_char = self.current_char();
        }
        self.advance();
        return Token {
            kind: token::TokenKind::STRING,
            content: Some(string),
            position: initial_index,
        };
    }
}
