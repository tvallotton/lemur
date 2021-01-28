use super::super::parser::ast::Primitive;
use rug;

use std::collections::HashMap;



    

// pub static RESERVED_SYMBOLS: [&str; 8] = [
//     "->", "=", 
//     ":=", "<-", 
//     "=>", ":", 
//     "|",  "..",

// ];

pub const KEYWORDS: [&str; 40] = [
    "let",
    "do",
    "where",
    "if",
    "then",
    "else",
    "data",
    "enum",
    "type",
    "import",
    "export",
    "as",
    "when",
    "case",
    "of",
    "impl",
    "trait",
    "module",
    "using",
    "alignment",
    "_",
    "in",
// for possible future use
    "try",
    "class",
    "instance",
    "except",
    "finally",
    "alias",
    "super",
    "pub",
    "end",
    "forall",
    "for",
    "while",


    "mut",

    // corroutines 
    "async",
    "await",

    // for future support of macros
    "syntax",
    "macro",
    "template"
];

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    FuncMacro(String),
    Variable(String),
    NamespaceCall(String, Box<Token>),
    Integer(rug::Integer),
    String(String),
    Char(char),
    Float(f64),
    Complex(f64),
    Bool(bool),
    Symbol(String), 
    Indentation(i32),

    // special tokens
    Comma,            // ,
    Hash,             // #
    VerticalLine,     // |
    OCurly,           // {
    CCurly,           // }
    OSquare,          // [
    CSquare,          // ]
    OParens,          // )
    CParens,          // (
    Colon,            // :
    RightArrow,       // ->
    LeftArrow,        // <-
    DoubleArrow,      // =>
    RangeOperator,    // ..
    Equals,           // =
    ColonEquals,      // :=

}

impl Token {
    fn to_primitive(self) -> Primitive {
        use Token::*;
        match self {
            Bool(b) => Primitive::Bool(b),
            Integer(i) => Primitive::Integer(i),
            Float(f) => Primitive::Float(f),
            String(s) => Primitive::String(s),
            Char(c) => Primitive::Char(c),
            Complex(c) => Primitive::Complex(c),
            _ => panic!(),
        }
    }
}








