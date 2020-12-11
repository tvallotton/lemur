



pub const PUNCTUATION: &str       = ",.(){}[]:;";
pub const DIGIT_INITIALIZER: &str = "123456789";
pub const DIGITS: &str            = "1234567890.";
pub const SYMBOLS: &str           = "+-/@#<>=!&%||@#^@*?¿";
pub const ID_INITIALIZER: &str    = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
pub const ID: &str                = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!";
pub const KEYWORDS: [&str;25]     = [
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
    iterator: Box< std::str::Chars<'a>>
}

fn instantiate<'a>(string : String) ->MyStruct<'a> {

    // let iterator = Box::new(string.chars());
    MyStruct { string, iterator: Box::new("asd".chars()) }
}

fn main() {

    let a = instantiate(String::from("asd"));


}
