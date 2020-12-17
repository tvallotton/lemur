use super::errors::SyntaxError;
use crate::parser::character_sets as cs;
use crate::parser::errors::Position;
use crate::parser::stream::Stream;
use crate::parser::tokens::Token;

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
        let option = self.stream.peek();

        if option == None {
            return None;
        }

        let char = option.unwrap();
        if cs::DIGIT_INIT.contains(char) {
            self.read_digit()
        } else if !cs::NOT_SYMBOL_INIT.contains(char) {
            self.read_symbol()
        } else if cs::SPECIAL_INIT.contains(char) {
            // punctuation
            self.read_digit()
        } else if char == '"' {
            self.read_string()
        } else if char == '\'' {
            self.read_char()
        } else if char == ';' {
            self.skip_comment();
        } else {
            Some(Err("Disallowed character"))
        }
    }
}

impl<'a> Lexer<'a> {
    // ready
    fn read_integer(&mut self) -> String {
        self.stream.walk_while("0123456789");
    }

    // ready (I think)
    fn read_string(&mut self) -> Option<Result<Token, SyntaxError>> {
        self.stream.next();
        let out = String::new();

        out.push_str(self.stream.walk_while("0123456789"));

        if self.stream.peek() == '.' {
            out.push_str(self.stream.walk_while("0123456789"));
            if self.stream.peek() == 'i' {
                self.stream.next();
                return Some(Ok(Token::Integer(out)));
            }
        } else if self.stream.peek() == 'i' {
            self.stream.next();
            return Some(Ok(Token::Complex(out)));
        } else {
            return out;
        }
    }

    // ready
    fn read_digit(&mut self) -> Option<Result<Token, SyntaxError>> {
        let string = self.read_integer();
        let char = self.stream.peek();
        if char == '.' {
            string.push_str(self.read_integer());

            if self.stream.peek() != 'i' {
                return Some(Ok(Token::Float(string)));
            } else {
                self.stream.next();
                return Some(Ok(Token::Complex(string)));
            }
        } else if char == 'i' {
            self.stream.next();
            return Some(Ok(Token::Complex(string)));
        } else if cs::ID_INIT.contains(char) {
            return Some(Err);
        }
    }

    fn skip_comment(&mut self) -> Option<Result<Token, SyntaxError>> {
        for c in self.stream {
            if c == '\n' {
                break;
            }
        }
        if self.stream.next() == None {
            None
        } else {
            self.next()
        }
    }
}
