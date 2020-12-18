pub const RESERVED_SYMBOLS: [&str; 8] = [
    "->", "=", ":=", "<-", "=>", //asd
    ":", "|", "..",
];

pub const KEYWORDS: [&str; 34] = [
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
    "instance",
    "module",
    "using",
    "infix",
    "precedence",
    "alignment",
    // for possible future use
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

pub enum Token {
    Keyword(String),
    FuncMacro(String),
    Variable(String),
    Integer(String),
    String(String),
    FString(String),
    Float(String),
    Complex(String),
    Bool(String),   // True || False
    Symbol(String), // !"·$%&/=^*|:"

    // special tokens
    Comma,
    Hash,
    VerticalLine,
    OCurly,
    OSquare,
    CSquare,
    OParens,
    CParens,
    Colon,
    Period,
}
