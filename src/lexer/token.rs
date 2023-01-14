#[derive(Debug)]
pub enum TokenKind {
    LPAREN,
    RPAREN,
    ID,
    STRING,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub content: Option<String>,
    pub position: usize,
}

pub fn is_quote(c: char) -> bool {
    return c == '\'' || c == '"';
}
