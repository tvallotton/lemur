

#[cfg(test)]
mod tests {
    use super::super::super::ast;
    use super::super::*;
    #[test]
    fn initialize_parser() {
        let _ = ModuleParser::new("", "main");
    }

    #[test]
    fn parse_import() {
        let string = "import library";

        let parser = ModuleParser::new_main(string);

        let output = parser.build();
    }
}
