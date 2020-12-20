use super::lexer::Lexer;

struct Parser<'a> {
    lexer: Lexer<'a>,
    string: &'a String,
    ast: AST,
}

impl<'a> Parser<'a> {
    fn build(&mut self) -> Result<AST, SyntaxError> {
        for result in self.lexer {
            if let Ok(token) = result {
                self.handle(token)
            } else {
                Err(result.display())
            }
        }
    }
    
    fn handle()

    fn parse_if()
    fn parse_data()
    fn parse_enum()
    fn parse_()
}






