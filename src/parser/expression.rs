#[derive(Debug, Clone)]
pub enum Expression {
    String(String),
    ID(ID),
}

#[derive(Debug, Clone)]
pub struct ID {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum ExpressionValue {
    String(String),
    Number(isize),
}
