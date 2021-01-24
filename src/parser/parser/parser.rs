use super::ast;
use crate::parser::errors::SyntaxError;
use crate::parser::lexer::tokens::Token;
use crate::parser::lexer::Lexer;

use ast::Identifier;

pub struct ModuleParser<'a> {
    lexer: Lexer<'a>,
    result: Result<ast::Module, Vec<SyntaxError>>,
}

impl<'a> ModuleParser<'a> {
    pub fn new(string: &'a String, name: &str) -> ModuleParser<'a> {
        ModuleParser {
            lexer: Lexer::new(string),
            result: Ok(ast::Module {
                name: name.to_owned(),
                imports: vec![],
                sub_modules: vec![],
                type_declarations: vec![],
                data_declarations: vec![],
                assignments: vec![],
            }),
        }
    }
    pub fn new_main(string: &'a String) -> ModuleParser<'a> {
        ModuleParser::new(string, "main")
    }

    fn peek_variable(&self) -> Result<ast::Variable, SyntaxError> {
        if let Token::Variable(var_name) = self.lexer.peek()? {
            Ok(var_name)
        } else {
            Err(self.unexpected_token(Some("an identifier.")))
        }
    }

    fn next_variable(&mut self) -> Result<ast::Variable, SyntaxError> {
        if let None = self.lexer.next() {
            Err(self.raise_eof())
        } else {
            self.peek_variable()
        }
    }

    fn raise_eof_message(&self, object: &str) -> SyntaxError {
        self
            .lexer
            .syntax_error(
            &format!("Unexpected end of file while parsing {}", 
                    object)
            )
        
    }
    fn raise_eof(&self) -> SyntaxError {
        self.raise_eof_message("Unexpected end of file.")
    }
    fn unexpected_token(&self, expected: Option<&str>) -> SyntaxError {
        if let Some(allowed) = expected {
            self.lexer
                .underline_last_token(&format!("Unexpected token. Expected {}", allowed))
        } else {
            self.lexer.underline_last_token("Unexpected token.")
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
