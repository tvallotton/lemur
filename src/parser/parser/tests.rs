

#[cfg(test)]
mod tests {
    
    use crate::parser::ast;
    use crate::parser::parser::parser::Parser;

    #[test]
    fn instatiate_parser() {
        
        let string = String::from("sdasd");
        let _ = Parser::new(&string);
    }


    
}
