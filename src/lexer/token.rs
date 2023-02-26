#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LParenthesis,
    RParenthesis,
    ID,
    String,
    Semicolon,
    Comma,
    Equals,
    KeywordVar
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub position: usize,
}

impl Token {
    pub fn is_kind(&self, kind: TokenKind) -> bool {
        return self.kind == kind;
    }
}

pub fn is_quote(c: char) -> bool {
    return c == '\'' || c == '"';
}
