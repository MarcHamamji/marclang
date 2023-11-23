use marclang::lexer::Lexer;
use marclang::parser::Parser;

pub fn parse(code: &str) {
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();
    println!("{:#?}", ast);
}

#[cfg(test)]
mod parser {
    use crate::parse;

    #[test]
    fn hello_world() {
        parse("
            print(\"Hello World!\");
        ")
    }

    #[test]
    fn hello_worlds() {
        parse("
            print(\"Hello World!\");
            print(\"Hello World!\");
        ")
    }

    #[test]
    fn var_assignment() {
        parse("
            var a = \"Hello\";
        ")
    }

    #[test]
    fn print_var() {
        parse("
            var a = \"Hello\";
            print(a);
        ")
    }

    #[test]
    fn print_no_args() {
        parse("
            print();
        ")
    }

    #[test]
    fn print_multiple_args() {
        parse("
            var a = \"World!\";
            print(\"Hello\", a);
        ");
    }
}
