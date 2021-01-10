#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod settings;

fn main() {
    let lexer = parser::lexer::Lexer::new(
        "data Complex {
    real: Float,
    imag: Float,
}",
    );

    for token in lexer {
        println!("{:?}", token.unwrap());
    }
}
