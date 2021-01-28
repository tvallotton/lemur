use super::ast;
use crate::parser::errors::SyntaxError;
use crate::parser::lexer::tokens::Token;
use crate::parser::lexer::Lexer;

use ast::Identifier;

use super::ModuleParser;

impl<'a> ModuleParser<'a> {
    // fn build(&mut self) {
    //     loop {
    //         self.build_next();

    //         if self.finished() {
    //             break;
    //         }
    //     }
    // }

    

    fn peek_variable(&self) -> Result<ast::Variable, SyntaxError> {
        if let Token::Variable(var_name) = self.lexer.peek()? {
            Ok(var_name)
        } else {
            Err(self.unexpected_token(Some("an identifier.")))
        }
    }

    // Move
    fn next_variable(&mut self) -> Result<ast::Variable, SyntaxError> {
        if let None = self.lexer.next() {
            Err(self.raise_eof())
        } else {
            self.peek_variable()
        }
    }
}
