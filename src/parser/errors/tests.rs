#[cfg(test)]
pub mod tests {
    use crate::parser::errors::*;

    #[test]
    fn simple_display() {
        let string = String::from(
            "asdhjrwqd
        hello world
        good by world
        ",
        );

        let expect =
            "  |\n2 |         hello world\n  |         ^^^^^^^^^^^\nSyntax Error: You stupid dog";
        let mut syntax_error = SyntaxError::new(&string, 1, (8, 19));
        syntax_error.message = String::from("You stupid dog");
        assert_eq!(syntax_error.simple_display(), expect);
    }
}
