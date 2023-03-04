use marclang::lexer::Lexer;
use marclang::parser::Parser;

pub fn run(code: &str) {
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();
    println!("{:#?}", ast);
}

#[cfg(test)]
mod parser {
    use crate::run;

    #[test]
    fn hello_world() {
        run("
            print(\"Hello World!\");
        ")
    }

    #[test]
    fn hello_worlds() {
        run("
            print(\"Hello World!\");
            print(\"Hello World!\");
        ")
    }

    #[test]
    fn var_assignment() {
        run("
            var a = \"Hello\";
        ")
    }

    #[test]
    fn print_var() {
        run("
            var a = \"Hello\";
            print(a);
        ")
    }

    #[test]
    fn print_no_args() {
        run("
            print();
        ")
    }

    #[test]
    fn print_multiple_args() {
        run("
            var a = \"World!\";
            print(\"Hello\", a);
        ");
    }
}
