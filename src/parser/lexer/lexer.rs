use super::character_sets as cs;
use crate::parser::errors::Position;
use crate::parser::errors::SyntaxError;
use crate::parser::stream::Stream;
use super::tokens;
use super::tokens::Token;

pub struct Lexer<'a> {
    string: &'a str,
    stream: Stream<'a>,
    checkpoint: Position,
    token: Result<Token, SyntaxError>,
    previous_checkpoint: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str) -> Lexer<'a> {
        Lexer {
            string: string,
            stream: Stream::from(string),
            token: Ok(Token::Indentation(0)),
            previous_checkpoint: Position::new(0, 0),
            checkpoint: Position::new(0, 0),
        }
    }

    fn set_checkpoint(&mut self) {
        self.previous_checkpoint = self.checkpoint;
        self.checkpoint = self.stream.pos();
    }

    pub fn highlight_last_token(&mut self, message: &str) -> SyntaxError {
        let pos = self.stream.pos();
        let col0 = self.previous_checkpoint.col;
        let col1 = pos.col;
        let mut error = SyntaxError::new(self.string, pos.row, (col0, col1 - 1));
        error.message = message.to_string();
        error
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
        self.token = if self.stream.eof() {
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
        }?;
        Some(self.token.clone())
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

            Ok(Token::Complex(number.parse().unwrap()))
        } else {
            Ok(Token::Integer(number.parse().unwrap()))
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
                            Ok(Token::Complex(number.parse().unwrap()))
                        } else {
                            number.push_str(&pow);
                            Ok(Token::Float(number.parse().unwrap()))
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
                Ok(Token::Complex(number.parse().unwrap()))
            } else {
                Ok(Token::Float(number.parse().unwrap()))
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
        if self.stream.next() == Some('|') {
            return Ok(Token::Symbol("||".to_string()));
        }
        match char {
            ',' => Ok(Token::Comma),
            '#' => Ok(Token::Hash),
            '{' => Ok(Token::OCurly),
            '}' => Ok(Token::CCurly),
            '[' => Ok(Token::OSquare),
            ']' => Ok(Token::CSquare),
            '(' => Ok(Token::OParens),
            ')' => Ok(Token::CParens),
            ':' => Ok(Token::Colon),
            '.' => Ok(Token::Period),
            '|' => Ok(Token::VerticalLine),
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
            } else if c == '\\' && !except {
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
