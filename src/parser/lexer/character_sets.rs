// #![warn(dead_code)]
// #![warn(unused_imports)]

// Initializers
pub const DIGIT_INIT     : &str = "123456789";
pub const FLOAT          : &str = "0123456789.";
pub const WHITE_SPACE    : &str = " ";
pub const ID_INIT        : &str = "asdfghjklqwertyuiopzxcvbnmASDFGHJKLQWERTYUIOPZXCVBNM";
pub const SPECIAL_INIT   : &str = ",.:(){}[]|#";
pub const CHAR_INIT      : &str = "'";
pub const NOT_SYMBOL_INIT: &str = "\n\t\r_qwertyuiopasd  
                                   fghjklzxcvbnmQWERTYUIOASD 
                                   FGHJKLZXCVBNM!1234567890. 
                                   ;,{}[]()”“'\\\"";

// body
pub const SPECIAL_TOKENS: &str = SPECIAL_INIT;
pub const INTEGER       : &str = "1234567890";
pub const NOT_SYMBOLS   : &str = NOT_SYMBOL_INIT;
pub const ID_INITIALIZER: &str = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
pub const ID            : &str = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";




pub fn is_digit(c: char) -> bool {
    DIGIT_INIT.contains(c)
}

pub fn is_identifier(c: char) -> bool {
    ID_INIT.contains(c)
}

pub fn is_special_token(c: char) -> bool {
    SPECIAL_INIT.contains(c)
}

pub fn is_string(c: char) -> bool {
    "\"".contains(c)
}
pub fn is_char(c: char) -> bool {
    "\'".contains(c)
}