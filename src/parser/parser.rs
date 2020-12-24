use super::ast;
use super::lexer::Lexer;
use super::tokens::Token;

struct Parser<'a> {
    lexer: Lexer<'a>,
    string: &'a String,
    ast: ast::Module,
}

impl<'a> Parser<'a> {
    fn new(string: &'a String, name: String) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(string),
            string,
            ast: ast::Module {
                name,
                imports: vec![],
                sub_modules: vec![],
                type_declarations: vec![],
                data_declarations: vec![],
                assignments: vec![],
            },
        }
    }
    fn new_main(string: &'a String) -> Parser<'a> {
        Parser::new(string, String::from("main"))
    }

    fn build(&self) {
        loop {

            match self.build_next()

        }
    }

    fn build_next(&self) -> Result<Module, SyntaxError> {
        let option = self.lexer.next();
        if let Some(result) = option {
            let value = result?
            


        }
    }
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
