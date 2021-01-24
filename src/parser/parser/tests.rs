

use super::ast;
use super::ModuleParser;

#[test]
fn instatiate_parser() {
    
    let string = String::from("sdasd");
    let _ = ModuleParser::new(&string, "module_name");
}



