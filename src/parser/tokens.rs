pub const RESERVED_SYMBOLS: [&str; 8] = [
    "->", "=", ":=", "<-", "=>", //asd
    ":", "|", "..",
];

pub const KEYWORDS: [&str; 37] = [
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

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    FuncMacro(String),
    Variable(String),
    Integer(String),
    String(String),
    FString(String),
    Char(String),
    Float(String),
    Complex(String),
    Bool(String),   // True || False
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
