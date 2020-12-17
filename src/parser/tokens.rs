
pub const RESERVED_SYMBOLS : [&str;8] = [
    "->",
    "=",
    ":=",
    "<-",
    "=>",
    ":",
    "|",
    ".."
];

pub const KEYWORDS: [&str;25] = [
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
    "macro"
];


 
pub enum Token {
    Keyword(String),
    Variable(String),
    Integer(String),
    String(String),
    FString(String),
    Float(String),
    Complex(String), 
    Bool(String), // True || False
    Punctuation(String), // . , ;
    Open(String), // { ( [
    Close(String), // } ) ]
    Symbol(String),      // !"·$%&/=^*|:"
}


