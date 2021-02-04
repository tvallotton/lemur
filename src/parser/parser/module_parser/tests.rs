#[cfg(test)]
mod tests {
    use super::super::super::ast;
    use super::super::*;
    use crate::parser::parser::ast::Module;
    use std::collections::HashMap;
    use str_macro::str;
    #[test]
    fn initialize_parser() {
        let _ = ModuleParser::new("");
    }

    #[test]
    fn parse_empty_string() {
        let string = "";

        let mut parser = ModuleParser::new(string);
        let output = parser.build();
        let expect = Ok(Module {
            assignments: vec![],
            data: vec![],
            imports: vec![],
            submodules: HashMap::new(),
            types: vec![],
        });
        assert_eq!(output, expect);
    }

    #[test]
    fn parse_variable() {
        let string = "my_var";
        let mut parser = ModuleParser::new_main(string);
        let variable = parser.parse_variable();
        let expect = Ok(str!("my_var"));
        assert_eq!(output, expect);
    }
}
