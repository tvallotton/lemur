pub const SPECIAL_TOKENS: &str = ",.(){}[]:;|#";
pub const DIGIT_INIT: &str = "123456789";
pub const INT: &str = "1234567890";
pub const FLOAT: &str = "1234567890.";
pub const SYMBOLS: &str = "+-/@<>=!&%|@^@*?¿";
pub const ID_INITIALIZER: &str = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
pub const ID: &str = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!";
pub const KEYWORDS: [&str; 25] = [
    "let",
    "where",
    "if",
    "then",
    "else",
    "precedence",
    "alignment",
    "data",
    "type",
    "import",
    "as",
    "from",
    "when",
    "try",
    "except",
    "finally",
    "for",
    "in",
    "async",
    "await",
    "while",
    "do",
    "loop",
    "mut",
    "pub",
];

struct MyStruct<'a> {
    string: String,
    iterator: Box<std::str::Chars<'a>>,
}

fn instantiate<'a>(string: String) -> MyStruct<'a> {
    // let iterator = Box::new(string.chars());
    MyStruct {
        string,
        iterator: Box::new("asd".chars()),
    }
}

fn main() {
    let a = instantiate(String::from("asd"));
}
