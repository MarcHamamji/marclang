#[derive(Debug)]
pub enum AST<'a> {
    VariableDeclaration(VariableDeclaration<'a>),
    Compound(Compound<'a>),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct VariableDeclaration<'a> {
    pub name: String,
    pub value: &'a AST<'a>,
}

#[derive(Debug)]
pub struct Compound<'a> {
    pub list: Vec<AST<'a>>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<String>,
}
