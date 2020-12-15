
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
    "from",
    "when",
    "case",
    "of",
    "try",
    "except",
    "finally",
    "instance",
    "precedence",
    "alignment",
    "module",
    
    // for possible future use
    "forall",
    "infix",
    "async",
    "await",
    "in",
    "with",
    "at",
    "syntax",
    "macro"
];



                    
enum Token {
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

    SyntaxError
}
