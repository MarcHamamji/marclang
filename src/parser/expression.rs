use std::fmt;

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

impl fmt::Display for ExpressionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionValue::String(value) => write!(f, "{}", value),
            ExpressionValue::Number(value) => write!(f, "{}", value.to_string()),
        }
    }
}
