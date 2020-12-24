#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod settings;

fn main() {

    let string = String::from("a sadj 3 dasoj");
    let lexer = parser::lexer::Lexer::new(&string);
    
    for token in lexer {
        println!("{:?}", token);
    }
    
}
