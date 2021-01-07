
#[cfg(test)]
mod tests {
    use crate::parser::ast;
    use crate::parser::parser::*;

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
