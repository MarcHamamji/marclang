#[cfg(test)]
mod hello_world {
    use marclang::lexer::Lexer;
    use marclang::parser::Parser;

    #[test]
    fn test() {
        let code = "
print(\"Hello World!\");
";
        let lexer = Lexer::new(&code);
        let mut parser = Parser::new(lexer);

        let _ast = parser.parse();
    }
}

