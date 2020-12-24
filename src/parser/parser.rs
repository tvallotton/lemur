use super::ast;
use super::lexer::Lexer;
use super::tokens::Token;

struct Parser<'a> {
    lexer: Lexer<'a>,
    string: &'a String,
    ast: ast::Module,
}

impl<'a> Parser<'a> {
    fn new(string: &'a String) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(string),
            string,
            ast: ast::Module {
                imports: vec![],
                type_declarations: vec![],
                data_declarations: vec![],
                assignments: vec![],
            },
        }
    }
}

// impl<'a> Parser<'a> {
//     fn build_next(&mut self) -> Result<AST, SyntaxError> {
//         let option = self.lexer.next();

//         if let Some(result) = option {
//             let token = result?;

//             if token == Token::Keyword("module") {
//                 self.handle_module();
//             }
//             // else if token == Token::Keyword("import") {
//             //     self.handle_import();
//             // }
//             // else if token == Token::Keyword("export") {
//             //     self.handle_import();
//             // }
//             // else if token == Token::Keyword("data") {
//             //     self.handle_data();
//             // }
//             // else if token == Token::Keyword("enum") {
//             //     self.handle_enum();
//             // }
//             // else if token == Token::Keyword("instance") {
//             //     self.handle_instance();
//             // }
//             // else if token == Token::Keyword("class") {
//             //     self.handle_class();
//             // }
//             // else if token == Token::Keyword("type") {
//             //     self.handle_type()
//             // }
//             else if let token = Token::Variable(name) {
//                 self.handle_variable(name)
//             }
//         }
//         else {
//             None
//         }?;
//     }

//     fn read_variable(&mut self) ->  {
//         if let Some(Ok(Token::Variable(name))) = self.lexer.next() {

//             Ok(ast::Expr::Primitive::Variable(name))

//         } else {

//         }
//     }

// }

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::parser::lexerLexer;

    #[test]
    fn instatiate_parser() {
        let string = String::from("sdasd");
        let parser = Parser::new(&string);
    }
}
