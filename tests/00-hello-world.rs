#[cfg(test)]
mod tests {
    use marclang::lexer::Lexer;
    use marclang::parser::Parser;

    #[test]
    fn test() {
        let code = "
var a = \"test\";
";
        let lexer = Lexer::new(&code);
        let mut parser = Parser::new(lexer);

        let _ast = parser.parse();
    }
}
