use std::collections::HashMap;

use crate::parser::ast::Compound;
use crate::parser::ast::FunctionCall;
use crate::parser::ast::VariableDeclaration;
use crate::parser::ast::AST;
use crate::parser::expression::Expression;
use crate::parser::expression::ExpressionValue;
use crate::parser::Parser;

mod variable;

#[derive(Debug)]
pub struct Visitor {
    parser: Parser,
    output: String,
    scope: HashMap<String, ExpressionValue>,
}

impl Visitor {
    pub fn new(parser: Parser) -> Self {
        Self {
            parser,
            output: String::new(),
            scope: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> &String {
        let ast = self.parser.parse();
        self.run_compound(ast);
        &self.output
    }

    fn run_compound(&mut self, compound: Compound) {
        compound.list.into_iter().for_each(|line| match line {
            AST::FunctionCall(function_call) => {
                self.run_function(function_call);
            }
            AST::VariableDeclaration(variable_declaration) => {
                self.declare_variable(variable_declaration);
            }
            _ => unimplemented!(),
        })
    }

    fn run_function(&mut self, function_call: FunctionCall) -> Option<Expression> {
        match function_call.function_name.as_str() {
            "print" => {
                function_call
                    .arguments
                    .into_iter()
                    .enumerate()
                    .for_each(|(index, expression)| {
                        if index != 0 {
                            self.output.push_str(" ")
                        }
                        match expression {
                            Expression::String(arg) => {
                                self.output += &arg;
                            }
                            Expression::ID(arg) => {
                                if self.scope.get(&arg.name).is_none() {
                                    panic!("Undefined variable '{}'", arg.name);
                                }
                                self.output += &self.scope.get(&arg.name).expect("").to_string();
                                return;
                            }
                        }
                    });
                self.output += "\n";
                None
            }
            _ => panic!("Unknown function '{}'", function_call.function_name),
        }
    }

    fn declare_variable(&mut self, variable_declaration: VariableDeclaration) {
        self.scope.insert(
            variable_declaration.name,
            self.evaluate_expression(variable_declaration.value),
        );
    }

    fn evaluate_expression(&self, expression: Expression) -> ExpressionValue {
        match expression {
            Expression::String(string) => ExpressionValue::String(string),
            Expression::ID(name) => self.read_variable(name.name),
        }
    }

    fn read_variable(&self, name: String) -> ExpressionValue {
        self.scope
            .get(&name)
            .expect(format!("Unknown variable '{}'", name).as_str())
            .to_owned()
    }
}
