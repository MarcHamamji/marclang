#[derive(Debug)]
pub enum Expression {
    String(String),
    ID(ID)
}

#[derive(Debug)]
pub struct ID {
    pub name: String
}
