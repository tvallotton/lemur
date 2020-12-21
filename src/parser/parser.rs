use super::lexer::Lexer;
use super::tokens::Token;

struct Parser<'a> {
    lexer: Lexer<'a>,
    string: &'a String,
    ast: AST,
}

impl<'a> Parser<'a> {
    fn build(&mut self) -> Result<AST, SyntaxError> {
        let option = self.lexer.next();

        if let Some(result) = option {
            let token = result?;

            if token == Token::Keyword("import") {
                self.handle_import();
            }
            if token == Token::Keyword("export") {
                self.handle_import();
            }
            if token == Token::Keyword("data") {
                self.handle_data();
            }
            if token == Token::Keyword("enum") {
                self.handle_enum();
            }
            if token == Token::Keyword("instance") {
                self.handle_instance();
            }
            if token == Token::Keyword("class") {
                self.handle_class();
            }
        if token == Token::Keyword("class") {
                self.handle_class();
            }
        }
    }
}
