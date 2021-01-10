

use super::ast;
use super::Parser;

#[test]
fn instatiate_parser() {
    
    let string = String::from("sdasd");
    let _ = Parser::new(&string);
}



