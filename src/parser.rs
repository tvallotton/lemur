



use crate::lexer::Lexer;










struct Parser <'a> {
    lexer: Lexer<'a>, 
}

impl<'a> Parser<'a> {

    fn new(string: &'a String) {
        return Parser {
            lexer: Lexer::new(),
        }
    }









    fn build_AST(&mut self)
















}