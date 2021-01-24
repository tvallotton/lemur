use super::super::parser::ast::Primitive;
use rug;

pub const RESERVED_SYMBOLS: [&str; 8] = [
    "->", "=", ":=", "<-", "=>", //asd
    ":", "|", "..",
];

pub const KEYWORDS: [&str; 38] = [
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
    "try",
    "except",
    "finally",
    "impl",
    "trait",
    "module",
    "using",
    "infix",
    "precedence",
    "alignment",
    "_",
    // for possible future use
    "super",
    "pub",
    "end",
    "forall",
    "for",
    "while",
    "mut",
    "async",
    "await",
    "in",
    "syntax",
    "macro",
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
    Symbol(String), //
    Indentation(i32),
    // special tokens
    Comma,
    Hash,
    VerticalLine,
    OCurly,
    CCurly,
    OSquare,
    CSquare,
    OParens,
    CParens,
    Colon,
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
