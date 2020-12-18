use super::character_sets as cs;
use super::errors::Position;
use super::errors::SyntaxError;
use super::stream::Stream;
use super::tokens;
use super::tokens::Token;

struct Lexer<'a> {
    string: &'a String,
    stream: Stream<'a>,
    checkpoint: Position,
}

impl<'a> Lexer<'a> {
    fn new(string: &'a String) -> Lexer<'a> {
        Lexer {
            string: string,
            stream: Stream::from(string),
            checkpoint: Position::new(0, 0),
        }
    }

    fn set_checkpoint(&mut self) {
        self.checkpoint = self.stream.pos();
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, SyntaxError>;

    fn next(&mut self) -> Option<Result<Token, SyntaxError>> {
        self.set_checkpoint();
        let mut char = String::from(self.stream.peek());

        if self.stream.eof() {
            None
        } else if cs::WHITE_SPACE.contains(&char) {
            Some(self.skip_white_space());
            self.next()
        } else if cs::DIGIT_INIT.contains(&char) {
            Some(self.read_digit(char))
        } else if cs::ID_INIT.contains(&char) {
            Some(self.read_identifier(char))
        } else if cs::SPECIAL_TOKENS.contains(&char) {
            Some(self.read_sepcial_tokens(char))
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
            Some(self.syntax_error("Tabs are not supported."))
        } else {
            Some(self.read_symbol(char))
        }
    }
}
// methods to define:
//     read_indent                      -> (indent)
//     skip_white_space                 -> ()
//     read_digit                       -> (int, float, complex)
//     read_identifier { read_fstring } -> (macro, keyword, variable, fstring??)
//     read_sepcial_tokens
//     read_string
//     read_char

impl<'a> Lexer<'a> {
    // ready to test
    fn skip_white_space(&mut self) {
        for c in self.stream {
            if !cs::WHITE_SPACE.contains(c) {
                break;
            }
        }
    }

    // ready to test
    fn read_indent(&mut self) -> Result<Token, SyntaxError> {
        let count: i32 = 0;
        for c in self.stream {
            if c != ' ' {
                break;
            }
            count += 1;
        }
        return Ok(Token::Indentation(count));
    }
    // ready to test
    // missing possible floats like 3.4345e345
    fn read_digit(&mut self, mut number: String) -> Result<Token, SyntaxError> {
        number.push_str(self.stream.walk_while(cs::INTEGER));
        let char = self.stream.peek();
        if char == '.' || char == 'e' {
            self.read_float(number)
        } else if self.stream.peek() == 'i' {
            self.stream.next();
            Ok(Token::Complex(number))
        } else {
            Ok(Token::Integer(number))
        }
    }

    fn read_float(&mut self, mut number: String) {
        if self.stream.peek() == '.' {
            number.push('.');
            number.push_str(self.stream.walk_while(cs::INTEGER));
            if self.stream.peek() == 'i' {
                self.stream.next();
                Ok(Token::Complex(number))
            } else {
                Ok(Token::Float(number))
            }
        }
        if self.stream.peek() == 'e' {
            self.push('e');
            match self.stream.next() {
                Some(c) => {
                    if c == '+' || c == '-' || cs::INTEGER.contains(c) {
                        number.push(c);

                        let pow = self.stream.walk_while(cs::INTEGER);
                        if pow.len() == 0 {
                            let message = String::from(
                                "Invalid float literal. An order of magnitud was expected.",
                            );
                            self.syntax_error(message)
                        } else {
                            number.push_str(pow);
                            Ok(Token::Float(number))
                        }
                    } else {
                        let message =
                            format!("Unexpected character '{}' while reading float literal", c);
                        self.syntax_error(message)
                    }
                }
                None => self.syntax_error("Unexpected end of file while parsing float literal."),
            }
        }
    }

    fn read_identifier(&mut self, mut out: String) -> Result<Token, SyntaxError> {
        out.push_str(self.stream.walk_while(cs::ID));

        if tokens::KEYWORDS.contains(out) {
            Ok(Token::Keyword(out))
        } else if self.stream.peek() == '!' {
            self.stream.next();
            Ok(Token::Macro(out))
        } else {
            Ok(Token::Variable(out))
        }
    }

    // fn read_sepcial_tokens(&mut self, out) -> Result<Token, SyntaxError> {}
    // fn read_indent(&mut self, out) -> Result<Token, SyntaxError> {}
}
