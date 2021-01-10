use super::ast;
use crate::parser::errors::SyntaxError;
use crate::parser::lexer::Lexer;
use crate::parser::lexer::tokens::Token;

use ast::Identifier;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    string: &'a String,
    result: Result<ast::Module, SyntaxError>,
}

impl<'a> Parser<'a> {
    fn new(string: &'a String, name: String) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(string),
            string,
            result: Ok(ast::Module {
                name,
                imports: vec![],
                sub_modules: vec![],
                type_declarations: vec![],
                data_declarations: vec![],
                assignments: vec![],
            }),
        }
    }
    fn new_main(string: &'a String) -> Parser<'a> {
        Parser::new(string, String::from("main"))
    }

    fn map(&mut self, func: fn(Token) -> Token) {
        
    }



    fn handle_import(&mut self) -> Result<ast::Import, SyntaxError> {
        let identifier  = self.read_identifier()?;
        if self.expect_keyword("user") {

            let pseudonym = self.read_variable()?;
        Ok(ast::Import {
                name: identifier,
                pseudonym: Some(pseudonym),
            })
        } else {
            Ok(ast::Import {
                name: identifier,
                pseudonum: None
            })
        }
        
    }

    fn expect_keyword(&mut self, kw: &str) -> bool {
        Some(Ok(Token::Keyword(kw.to_string()))) == self.lexer.next()
    }

    fn read_variable(&self) -> Result<Token, SyntaxError> {
        if let Some(Ok(Token::Variable(name))) = self.lexer.next() {
            Ok(Token::Variable(name))
        } else {
            Err(self.lexer.highlight_last_token("an variable name."))
        }}
            
    

    fn read_identifier(&mut self) -> Result<ast::Identifier, SyntaxError> {
        match self.lexer.next() {
            Some(Ok(token)) => {
                if let Token::NamespaceCall(_, _) = &token {
                    Ok(token)
                } else {
                    self.lexer.highlight_last_token("an identifier.")
                }
            }

            Some(error) => error,
            _ => self.raise_eof()
        }
        
    }



    fn raise_eof(&self, object: &str) -> Err(SyntaxError) {
        self.lexer
            .highlight_last_token(format!("Unexpected end of file while parsing {}", object))
    }
    fn unexpected_token(&self, expected: Option<&str>) -> Err(SyntaxError) {
        if let Some(allowed) = expected {
            self.lexer
                .highlight_last_token(format!("Unexpected token. Expected {}", allowed))
        } else {
            self.lexer.highlight_last_token("Unexpected token.")
        }
    }


    // fn build_next(&self) -> Result((), SyntaxError) {
    //     let option = self.lexer.peek();
    //     if let Some(result) = option {
    //         let token = result?;
    //         if token == Token::Keyword("import") {
    //             let import = self.handle_import()?;
    //             self.ast.imports.push(import);
    //             // } else if token == Token::Keyword("import") {
    //             //     import = self.handle_import()?;
    //             //     self.ast.imports.push(import);
    //             // } else if token == Token::Keyword("export") {
    //             //     export = self.handle_export()?;
    //             //     self.ast.exports.push(export);
    //             // } else if token == Token::Keyword("data") {
    //             //     data = self.handle_data()?;
    //             //     self.ast.data_declarations.push(enum_);
    //             // } else if token == Token::Keyword("enum") {
    //             //     enum_ = self.handle_enum()?;
    //             //     self.ast.data_declarations.push(enum_);
    //             // } else if token == Token::Keyword("trait") {
    //             //     class = self.handle_class()?;
    //             //     self.ast.trait_declarations.push(class);
    //             // } else if token == Token::Keyword("type") {
    //             //     type_ = self.handle_instance()?;
    //             //     self.ast.data_declarations.push(type_);
    //             // } else if token == Token::Keyword("impl") {
    //             //     impl_ = self.handle_instance()?;
    //             //     self.ast.impl_declarations.push(impl_);
    //             // } else if let Token::Variable(var) = token {
    //             //     declaration = handle_variable()?;
    //         }
    //     } else {
    //         Err(self.unexpected_token());
    //     }
    // }
}
