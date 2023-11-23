use marclang::lexer::Lexer;
use marclang::parser::Parser;
use marclang::visitor::Visitor;

pub fn run(code: &str) -> String {
    let lexer = Lexer::new(code.to_string());
    let parser = Parser::new(lexer);
    let mut visitor = Visitor::new(parser);

    let output = visitor.run();
    output.clone()
}

#[cfg(test)]
mod visitor {
    use crate::run;

    #[test]
    fn hello_world() {
        assert_eq!(
            run("
                print(\"Hello World!\");
            "),
            "Hello World!\n"
        );
    }

    #[test]
    fn hello_comma_world() {
        assert_eq!(
            run("
                print(\"Hello\", \"World!\");
            "),
            "Hello World!\n"
        );
    }

    #[test]
    fn hello_worlds() {
        assert_eq!(
            run("
                print(\"Hello World!\");
                print(\"Hello World!\");
            "),
            "Hello World!\nHello World!\n"
        );
    }

    #[test]
    fn print_var() {
        assert_eq!(
            run("
                var a = \"Hello\";
                print(a);
            "),
            "Hello\n"
        );
    }

    #[test]
    fn print_var_and_string() {
        assert_eq!(
            run("
                var a = \"Hello\";
                print(a, \"World!\");
            "),
            "Hello World!\n"
        );
    }

    #[test]
    fn print_var_and_var() {
        assert_eq!(
            run("
                var a = \"Hello\";
                var b = \"World!\";
                print(a, b);
            "),
            "Hello World!\n"
        );
    }
}
