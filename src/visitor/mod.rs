use crate::parser::Parser;

#[derive(Debug)]
pub struct Visitor {
    parser: Parser,
}

impl Visitor {
    fn new(parser: Parser) -> Self {
        Self {
            parser
        }
    }
}
