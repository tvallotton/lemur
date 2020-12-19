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
    type Item = Result<Token, SyntaxError>;

    fn next(&mut self) -> Option<Result<Token, SyntaxError>> {
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
    fn read_indent(&mut self) -> Result<Token, SyntaxError> {
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
    fn read_digit(&mut self) -> Result<Token, SyntaxError> {
        let mut number = String::new();
        number.push_str(&self.stream.walk_while(cs::INTEGER));
        let char = self.stream.peek();
        if char == '.' || char == 'e' || char == 'E' {
            self.read_float(number)
        } else if self.stream.peek() == 'i' {
            self.stream.next();
            Ok(Token::Complex(number))
        } else {
            Ok(Token::Integer(number))
        }
    }

    fn read_float(&mut self, mut number: String) -> Result<Token, SyntaxError> {
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
                        let pow = self.stream.walk_while(cs::INTEGER);
                        if pow.len() == 0 {
                            let message = String::from(
                                "Invalid float literal. An order of magnitud was expected.",
                            );
                            Err(self.syntax_error(&message))
                        } else {
                            number.push_str(&pow);
                            Ok(Token::Float(number))
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
                Ok(Token::Complex(number))
            } else {
                Ok(Token::Float(number))
            }
        }
    }

    fn read_identifier(&mut self) -> Result<Token, SyntaxError> {
        let mut out = String::new();
        out.push_str(&self.stream.walk_while(&cs::ID));

        if tokens::KEYWORDS.contains(&&*out) {
            Ok(Token::Keyword(out))
        } else if self.stream.peek() == '!' {
            self.stream.next();
            Ok(Token::FuncMacro(out))
        } else {
            Ok(Token::Variable(out))
        }
    }

    fn read_sepcial_tokens(&mut self) -> Result<Token, SyntaxError> {
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
    fn read_string(&mut self) -> Result<Token, SyntaxError> {
        let mut out = String::new();
        let mut except = false;
        let mut eof = true;
        for c in &mut self.stream {
            if c == '"' && !except {
                eof = false;
                break;
            } else if c == '\\' {
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
            Ok(Token::String(out))
        }
    }

    fn read_char(&mut self) -> Result<Token, SyntaxError> {
        let mut out = String::new();
        let mut except = false;
        let mut eof = true;
        for c in &mut self.stream {
            if c == '\'' && !except {
                eof = false;
                break;
            } else if c == '\\' {
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
            Ok(Token::Char(out))
        }
    }

    fn skip_comment(&mut self) {
        for c in &mut self.stream {
            if c == '\n' {
                break;
            }
        }
    }

    fn read_symbol(&mut self, mut out: String) -> Result<Token, SyntaxError> {
        for c in &mut self.stream {
            if !cs::NOT_SYMBOLS.contains(c) {
                out.push(c);
            } else {
                break;
            }
        }
        Ok(Token::Symbol(out))
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
    #[test]
    fn instatiating_lexer() {
        let string = String::from("variable  \"the string\" 234 3.34e-34 345.34i 34i\n   \'a\',.|{}[]#():; comentario asd asd\nfin_comentario!  ");
        let mut lexer = Lexer::new(&string);
        let output = vec![
            Token::Indentation(0),
            Token::Variable("variable".to_string()),
            Token::String("the string".to_string()),
            Token::Integer("234".to_string()),
            Token::Float("3.34e-34".to_string()),
            Token::Complex("345.34".to_string()),
            Token::Complex("34".to_string()),
            Token::Indentation(3),
            Token::Char("a".to_string()),
            Token::Comma,
            Token::Period,
            Token::VerticalLine,
            Token::OCurly,
            Token::CCurly,
            Token::OSquare,
            Token::CSquare,
            Token::Hash,
            Token::OParens,
            Token::CParens,
            Token::Colon,
            Token::Indentation(0),
            Token::FuncMacro("fin_comentario".to_string()),
        ];
        for (item, expected) in lexer.zip(output) {
            match item {
                Err(error) => {
                    println!("Error: {:?}", error);
                }
                Ok(value) => {
                    if expected != value {
                        println!("ERROR: expected {:?} != got {:?}", expected, value);
                    } else {
                        println!("OK: expected {:?} == got {:?}", expected, value);
                    }
                }
            }
        }
    }
}
