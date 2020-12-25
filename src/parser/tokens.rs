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

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Keyword(&'a str),
    FuncMacro(&'a str),
    Variable(&'a str),
    Integer(&'a str),
    String(&'a str),
    FString(&'a str),
    Char(&'a str),
    Float(&'a str),
    Complex(&'a str),
    Bool(&'a str),   // True || False
    Symbol(&'a str), // 
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

