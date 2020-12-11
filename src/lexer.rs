
mod stream;

use stream::{
    Stream,
    ErrorHighlight,
    Position,
    Line,
};


struct Lexer<'a> {
    stream: Stream<'a>,
    check_point: Position, // marks where the token began
}


enum Keyword {
    lambda_,
    define_,


}

enum Token {
    keyword(Keyword),
    float(String),
    variable(String),
    boolean(bool),
    decimal(f64),
    integer(i32),
    symbol()

}


//
// impl<'a> Lexer<'a> {
//
//
//     fn next(&self) -> Token {
//
//
//
//     }
//
//
// }


fn main() {
    let string = "1341 23sda s3dasd".to_string();


    for substring in string.split(" ") {
        println!("{:?}", substring);
    }


}
