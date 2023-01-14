use token::Token;
mod token;

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

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        while self.index < self.chars.len() {
            let current_char = self.current_char();
            match current_char {
                ' ' | '\n' => self.advance(),
                '(' => {
                    tokens.push(Token {
                        kind: token::TokenKind::LPAREN,
                        position: self.index,
                        content: None,
                    });
                    self.advance()
                }
                ')' => {
                    tokens.push(Token {
                        kind: token::TokenKind::RPAREN,
                        position: self.index,
                        content: None,
                    });
                    self.advance()
                }
                '\'' | '"' => {
                    tokens.push(self.collect_string_and_advance());
                }
                c if c.is_ascii_alphanumeric() => tokens.push(self.collect_id_and_advance()),
                _ => panic!(
                    "Unexpected token `{}` at position {}",
                    current_char, self.index
                ),
            }
        }
        tokens
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
            println!("traversing string {}", self.index);
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
