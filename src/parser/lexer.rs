use crate::stream::Position;
use crate::stream::Stream;
use crate::tokens::Token;
use crate::parser::errors::SyntaxError;
use crate::character_sets as cs;

struct Lexer<'a> {
    string: &String<'a>,
    stream: Stream<'a>,
    checkpoint: Position,
}

impl<'a> Lexer<'a> {
    fn new(string: &'a String) -> Lexer<'a> {
        Lexer {
            stream: Stream::new(&string),
            checkpoint: Position::new(0, 0),
        }
    }

    fn set_checkpoint(&mut self) {
        self.checkpoint = self.stream.pos();
    }
}

impl Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Result<Token, SyntaxError>> {
        self.set_checkpoint();
        let option = self.stream.peek();

        if option == None {
            return None;
        } else if cs::DIGIT_INIT.contains(char) {
            self.read_digit()
        } else if cs::SYMBOL_INIT.contains(char) {
            self.read_symbol()
        } else if cs::SPECIAL_INIT.contains(char) {
            // punctuation
            self.read_digit()
        } else if cs::STRING_INIT.contains(char) {
            self.read_string()
        } else if cs::CHAR_INIT.contains(char) {
            // 'a'
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
        let out = String::from(self.stream.peek());
        loop {
            let option = self.stream.next();

            if option != None {
                let char = option.unwrap();
                if cs::INT.contains(char) {
                    out.push(char);
                    continue;
                } else {
                    return out;
                }
            } else {
                return out;
            }
        }
    }

    // ready (I think)
    fn read_string(&mut self) -> Option<Result<Token, SyntaxError>> {
        self.stream.next();
        let out = String::new();
        loop {
            let option = self.stream.next();
            if option == None {
                return None;
            }
            let char = option.unwrap();

            if char == '\\' {
                out.push(char);
                continue;
            }
            if char == '"' {
                self.stream.next();
                return Some(Ok(Token::String(out)));
            }
            out.push(char);
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

    fn skip_comment(&mut self) -> Option<Result<Token, SyntaxError>>{

        l
    
    }


}
