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
    Char(String),
    Float(f64),
    Complex(f64),
    Bool(bool),   // True || False
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
    Period,
}


