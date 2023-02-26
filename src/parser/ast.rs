#[derive(Debug)]
pub enum AST {
    VariableDeclaration(VariableDeclaration),
    Compound(Compound),
    FunctionCall(FunctionCall),
    // TEMPORARY - TO REMOVE
    String(String),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Box<AST>,
}

#[derive(Debug)]
pub struct Compound {
    pub list: Vec<AST>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<String>,
}
