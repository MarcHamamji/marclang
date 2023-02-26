#[cfg(test)]
mod var_assignment {
    use marclang::lexer::Lexer;
    use marclang::parser::Parser;

    #[test]
    fn test() {
        let code = "
var a = \"Hello\";
print(a);
";
        let lexer = Lexer::new(&code);
        let mut parser = Parser::new(lexer);

        let _ast = parser.parse();
    }
}

