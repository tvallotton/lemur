// use super::ast;
// use super::errors::SyntaxError;
// use super::lexer::Lexer;
// use super::tokens::Token;

struct Parser<'a> {
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

    fn build(&self) {}

    fn build_next(&self) -> Result((), SyntaxError) {
        let option = self.lexer.peek();
        if let Some(result) = option {
            let token = result?;
            if token == Token::Keyword("import") {
                import = self.handle_import()?;
                self.ast.imports.push(import);
                // } else if token == Token::Keyword("import") {
                //     import = self.handle_import()?;
                //     self.ast.imports.push(import);
                // } else if token == Token::Keyword("export") {
                //     export = self.handle_export()?;
                //     self.ast.exports.push(export);
                // } else if token == Token::Keyword("data") {
                //     data = self.handle_data()?;
                //     self.ast.data_declarations.push(enum_);
                // } else if token == Token::Keyword("enum") {
                //     enum_ = self.handle_enum()?;
                //     self.ast.data_declarations.push(enum_);
                // } else if token == Token::Keyword("trait") {
                //     class = self.handle_class()?;
                //     self.ast.trait_declarations.push(class);
                // } else if token == Token::Keyword("type") {
                //     type_ = self.handle_instance()?;
                //     self.ast.data_declarations.push(type_);
                // } else if token == Token::Keyword("impl") {
                //     impl_ = self.handle_instance()?;
                //     self.ast.impl_declarations.push(impl_);
                // } else if let Token::Variable(var) = token {
                //     declaration = handle_variable()?;
            }
        }
    }

    fn handle_import(&mut self) -> Result<ast::Import, SyntaxError> {
        let identifier = self.read_identifier()?;
    }

    fn read_identifier(&mut self) -> Result<ast::Identifier, SyntaxError> {
        let out;
        let option = self.lexer.next();
        if let Some(token) = option {
            if let Token::Keyword("super") = token {
                out = Identifier {
                    parent: "super",
                    child: vec![],
                };
            } else if let Token::Variable(var) = token {
                out = Identifier {
                    parent: var,
                    child: vec![],
                };
            }
            while let Some(result) = self.lexer.next() {
                let token = result?;
            }
            return Err(self.unexpected_token("an identifier."));
        } else {
            Err(self.raise_eof())
        }
        while let Some(result) = self.lexer.next() {}
    }

    fn raise_eof(&self, object: &str) -> SyntaxError {
        self.lexer
            .highlight_last_token(format!("Unexpected end of file while parsing {}", object))
    }
    fn unexpected_token(&self, expected: &str) -> SyntaxError {
        self.lexer
            .highlight_last_token(format!("Unexpected token. Expected {}", allowed))
    }
}

// fn raise_error(&mut self, message: String) -> SyntaxError {
//     self.lexer.syntax_error("Unexpected con")
// }
