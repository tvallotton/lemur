


pub const KEYWORDS: [&str;25] = [
    "let",
    "do",
    "where",
    "if",
    "then",
    "else",
    "data",
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
    "forall",
    "async",
    "await",
    "module",
    "instance",
    "precedence",
    "alignment",
    "with",
    "at",
];

asd sad asd






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
    Symbol(String),      // !"·$%&/=^*|"


}
