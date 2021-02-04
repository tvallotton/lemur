mod impl_module_parser;
mod tests;
use super::ast;
use crate::parser::errors::SyntaxError;
use crate::parser::lexer::tokens::Token;
use crate::parser::lexer::Lexer;

use ast::Identifier;

pub struct ModuleParser<'a> {
    lexer: Lexer<'a>,
    finished: bool,
}

impl<'a> ModuleParser<'a> {
    pub fn new(string: &'a str) -> ModuleParser<'a> {
        ModuleParser {
            lexer: Lexer::new(string),
            finished: false,
        }
    }
    pub fn new_main(string: &'a str) -> ModuleParser<'a> {
        ModuleParser::new(string)
    }

    fn raise_eof_message(&self, object: &str) -> SyntaxError {
        self.lexer
            .syntax_error(&format!("Unexpected end of file while parsing {}", object))
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

    // fn add_import(&mut self, result: Result<ast::Type, SyntaxError>) {
    //     match result {
    //         Ok(decl) => match &mut self.result {
    //             Ok(module) => module.types.push(decl),
    //             _ => {}
    //         },
    //         Err(error) => match &mut self.result {
    //             Ok(_) => self.result = Err(vec![error]),
    //             Err(errors) => errors.push(error),
    //         },
    //     }
    // }
    // fn add_typedeclaration(&mut self, result: Result<ast::Import, SyntaxError>) {
    //     match result {
    //         Ok(import) => match &mut self.result {
    //             Ok(module) => module.imports.push(import),
    //             _ => {}
    //         },
    //         Err(error) => match &mut self.result {
    //             Ok(_) => self.result = Err(vec![error]),
    //             Err(errors) => errors.push(error),
    //         },
    //     }
    // }
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
