use super::ast;
use super::errors::SyntaxError;
use super::lexer::Lexer;
use super::tokens::Token;

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
        let option = self.lexer.next();
        if let Some(result) = option {
            let token = result?;
            if token == Token::Keyword("module") {
                module = self.handle_module()?;
                self.ast.sub_modules.push(module);
            } else if token == Token::Keyword("import") {
                import = self.handle_import()?;
                self.ast.imports.push(import);
            } else if token == Token::Keyword("export") {
                export = self.handle_export()?;
                self.ast.exports.push(export);
            } else if token == Token::Keyword("data") {
                data = self.handle_data()?;
                self.ast.data_declarations.push(enum_);
            } else if token == Token::Keyword("enum") {
                enum_ = self.handle_enum()?;
                self.ast.data_declarations.push(enum_);
            } else if token == Token::Keyword("trait") {
                class = self.handle_class()?;
                self.ast.trait_declarations.push(class);
            } else if token == Token::Keyword("type") {
                type_ = self.handle_instance()?;
                self.ast.data_declarations.push(type_);
            } else if token == Token::Keyword("impl") {
                impl_ = self.handle_instance()?;
                self.ast.impl_declarations.push(impl_);
            } else if let Token::Variable(var) = token {
                declaration = handle_variable()?;
            }
        }
    }


    fn 

    fn raise_error(&mut self, message: String) {}
}

#[cfg(test)]
mod tests {
    use super::ast;
    use super::*;

    #[test]
    fn instatiate_parser() {
        let string = String::from("sdasd");
        let _ = Parser::new(&string);
    }

    #[test]
    fn build() {
        let string = String::from("module MyModule where");
        let parser = Parser::new(&string);

        let module = parser.build();

        let expected = ast::Module {
            name: name,
            imports: vec![],
            sub_modules: vec![],
            type_declarations: vec![],
            data_declarations: vec![],
            assignments: vec![],
        };
        expected.sub_modules.push(ast::Module {
            name: "MyModule",
            imports: vec![],
            sub_modules: vec![],
            type_declarations: vec![],
            data_declarations: vec![],
            assignments: vec![],
        });

        assert!(module == expected);
    }
}
