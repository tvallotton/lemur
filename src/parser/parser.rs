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

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::parser::lexerLexer;

    #[test]
    fn instatiate_parser() {
        let string = String::from("sdasd");
        let _ = Parser::new(&string);
    }
}
