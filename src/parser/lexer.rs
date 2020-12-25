use super::character_sets as cs;
use super::errors::Position;
use super::errors::SyntaxError;
use super::stream::Stream;
use super::tokens;
use super::tokens::Token;



pub struct Lexer<'a> {
    string: &'a str,
    stream: Stream<'a>,
    checkpoint: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str) -> Lexer<'a> {
        Lexer {
            string: string,
            stream: Stream::from(string),
            checkpoint: Position::new(0, 0),
            index: usize,
        }
    }

    fn set_checkpoint(&mut self) {
        self.checkpoint = self.stream.pos();
        
    }

    fn syntax_error(&self, message: &str) -> SyntaxError {
        let pos = self.stream.pos();
        let col0 = self.checkpoint.col;
        let col1 = pos.col;
        let mut error = SyntaxError::new(self.string, pos.row, (col0, col1));
        error.message = message.to_string();
        error
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, SyntaxError>;

    fn next(&mut self) -> Option<Result<Token<'a>, SyntaxError>> {
        self.set_checkpoint();
        let char = String::from(self.stream.peek());

        if self.stream.eof() {
            None
        } else if cs::WHITE_SPACE.contains(&char) {
            Some(self.skip_white_space());
            self.next()
        } else if cs::DIGIT_INIT.contains(&char) {
            Some(self.read_digit())
        } else if cs::ID_INIT.contains(&char) {
            Some(self.read_identifier())
        } else if cs::SPECIAL_TOKENS.contains(&char) {
            Some(self.read_sepcial_tokens())
        } else if char == "\n" {
            Some(self.read_indent())
        } else if char == "\"" {
            Some(self.read_string())
        } else if char == "'" {
            Some(self.read_char())
        } else if char == ";" {
            self.skip_comment();
            self.next()
        } else if char == "\t" {
            Some(Err(self.syntax_error("Tabs are not supported.")))
        } else {
            Some(self.read_symbol(char))
        }
    }
}

impl<'a> Lexer<'a> {
    // ready to test
    fn skip_white_space(&mut self) {
        for c in &mut self.stream {
            if !cs::WHITE_SPACE.contains(c) {
                break;
            }
        }
    }

    // ready to test
    fn read_indent(&mut self) -> Result<Token<'a>, SyntaxError> {
        let mut count: i32 = 0;
        for c in &mut self.stream {
            if c != ' ' {
                break;
            }
            count += 1;
        }
        return Ok(Token::Indentation(count));
    }
    // ready to test
    fn read_digit(&mut self) -> Result<Token<'a>, SyntaxError> {
        let mut number = String::new();
        number.push_str(&self.stream.walk_while(cs::INTEGER));
        let char = self.stream.peek();
        if char == '.' || char == 'e' || char == 'E' {
            self.read_float(number)
        } else if self.stream.peek() == 'i' {
            self.stream.next();
            Ok(Token::Complex(&number))
        } else {
            Ok(Token::Integer(&number))
        }
    }

    fn read_float(&mut self, mut number: String) -> Result<Token<'a>, SyntaxError> {
        if self.stream.peek() == '.' {
            number.push('.');
            for c in &mut self.stream {
                if cs::FLOAT.contains(c) {
                    number.push(c);
                } else {
                    break;
                }
            }
        }
        let peek = self.stream.peek();
        if peek == 'e' || peek == 'E' {
            number.push('e');
            match self.stream.next() {
                Some(c) => {
                    if c == '+' || c == '-' || cs::INTEGER.contains(c) {
                        if cs::INTEGER.contains(c) {
                            number.push('+');
                        }
                        let pow = self.stream.walk_while(cs::INTEGER);
                        if pow.len() == 0 {
                            let message = String::from(
                                "Invalid float literal. An order of magnitud was expected.",
                            );
                            Err(self.syntax_error(&message))
                        } else if self.stream.peek() == 'i' {
                            number.push_str(&pow);
                            self.stream.next();
                            Ok(Token::Complex(&number))
                        } else {
                            number.push_str(&pow);
                            Ok(Token::Float(&number))
                        }
                    } else {
                        let message =
                            format!("Unexpected character '{}' while reading float literal", c);
                        Err(self.syntax_error(&message))
                    }
                }
                _ => Err(self.syntax_error("Unexpected end of file while parsing float literal.")),
            }
        } else {
            if self.stream.peek() == 'i' {
                self.stream.next();
                Ok(Token::Complex(&number))
            } else {
                Ok(Token::Float(&number))
            }
        }
    }

    fn read_identifier(&mut self) -> Result<Token<'a>, SyntaxError> {
        let mut out = String::new();
        out.push_str(&self.stream.walk_while(&cs::ID));

        if tokens::KEYWORDS.contains(&&*out) {
            Ok(Token::Keyword(&out))
        } else if self.stream.peek() == '!' {
            self.stream.next();
            Ok(Token::FuncMacro(&out))
        } else {
            Ok(Token::Variable(&out))
        }
    }

    fn read_sepcial_tokens(&mut self) -> Result<Token<'a>, SyntaxError> {
        let char = self.stream.peek();
        self.stream.next();
        match char {
            ',' => Ok(Token::Comma),
            '#' => Ok(Token::Hash),
            '|' => Ok(Token::VerticalLine),
            '{' => Ok(Token::OCurly),
            '}' => Ok(Token::CCurly),
            '[' => Ok(Token::OSquare),
            ']' => Ok(Token::CSquare),
            '(' => Ok(Token::OParens),
            ')' => Ok(Token::CParens),
            ':' => Ok(Token::Colon),
            '.' => Ok(Token::Period),
            _ => panic!("This should never happen."),
        }
    }
    fn read_string(&mut self) -> Result<Token<'a>, SyntaxError> {
        let mut out = String::new();
        let mut except = false;
        let mut eof = true;
        for c in &mut self.stream {
            if c == '"' && !except {
                eof = false;
                break;
            } else if c == '\\' && !except  {
                except = true;
            } else {
                except = false;
                out.push(c);
            }
        }
        self.stream.next();
        if eof {
            Err(self.syntax_error("Unexpected end of file while parsing string literal."))
        } else {
            Ok(Token::String(&out))
        }
    }

    fn read_char(&mut self) -> Result<Token<'a>, SyntaxError> {
        let mut out = String::new();
        let mut except = false;
        let mut eof = true;
        for c in &mut self.stream {
            if c == '\'' && !except {
                eof = false;
                break;
            } else if c == '\\' && !except {
                except = true;
            } else {
                except = false;
                out.push(c);
            }
        }
        self.stream.next();
        if eof {
            Err(self.syntax_error("Unexpected end of file while parsing character literal."))
        } else {
            Ok(Token::Char(&out))
        }
    }

    fn skip_comment(&mut self) {
        for c in &mut self.stream {
            if c == '\n' {
                break;
            }
        }
    }

    fn read_symbol(&mut self, mut out: String) -> Result<Token<'a>, SyntaxError> {
        for c in &mut self.stream {
            if !cs::NOT_SYMBOLS.contains(c) {
                out.push(c);
            } else {
                break;
            }
        }
        Ok(Token::Symbol(&out))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::errors::SyntaxError;
    use crate::parser::tokens::Token;

    // things to test
    // 1) new
    // 2) ints
    // 3) floats
    // 4) strings
    // 5) chars
    // 6) an entire program
    // 7) keywords
    // 8) variables
    fn single_token<'a>(string: &'a str) -> Result<Token<'a>, SyntaxError> {
        let s = String::from(string);
        let mut lexer = Lexer::new(&s);
        lexer.next();
        lexer.next().unwrap()
    }
    #[test]
    fn string_token() {
        let result = single_token("\"string\\\"sad\"");
        let expect = Token::String("string\"sad");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn char_token() {
        let result = single_token("'a'");
        let expect = Token::Char("a");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn scape_char() {
        let result = single_token("'\\\\'");  // equivalente to lemur '\\'
        let expect = Token::Char("\\");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!("Got {:?}", result)
        }
    }
    #[test]
    fn variable_token() {
        let result = single_token("variable_123");
        let expect = Token::Variable("variable_123");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn integer_token() {
        let result = single_token("1234567890");
        let expect = Token::Integer("1234567890");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token0() {
        let result = single_token("1.234e-304-");
        let expect = Token::Float("1.234e-304");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token1() {
        let result = single_token("987e-654e");
        let expect = Token::Float("987e-654");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token2() {
        let result = single_token("124e+304+");
        let expect = Token::Float("124e+304");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token3() {
        let result = single_token("13.0987654321_");
        let expect = Token::Float("13.0987654321");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token4() {
        let result = single_token("124E-34");
        let expect = Token::Float("124e-34");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token5() {
        let result = single_token("13e34");
        let expect = Token::Float("13e+34");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn float_token6() {
        let result = single_token("13E44");
        let expect = Token::Float("13e+44");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token0() {
        let result = single_token("136i");
        let expect = Token::Complex("136");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token1() {
        let result = single_token("13.09876i");
        let expect = Token::Complex("13.09876");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token2() {
        let result = single_token("213e3i");
        let expect = Token::Complex("213e+3");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn complex_token3() {
        let result = single_token("21.3e3i");
        let expect = Token::Complex("21.3e+3");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }
    #[test]
    fn symbol_token0() {
        let result = single_token("¢∞¬÷654");
        let expect = Token::Symbol("¢∞¬÷");
        if let Ok(token) = result {
            assert!(token == expect, "found {:?}, expected {:?}", token, expect)
        } else {
            panic!()
        }
    }

}



