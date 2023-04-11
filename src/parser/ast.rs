use super::expression::Expression;

#[derive(Debug)]
pub enum AST {
    VariableDeclaration(VariableDeclaration),
    Compound(Compound),
    FunctionCall(FunctionCall),
    Expression(Expression),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Compound {
    pub list: Vec<AST>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<Expression>,
}
