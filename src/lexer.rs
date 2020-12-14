





use stream::Stream;
use stream::Position;





struct Lexer<'a> {
    stream: Stream<'a>,
    checkpoint: Position,
}


impl<'a> Lexer<'a> {
    fn new(string: &'a String) -> Lexer<'a> {
        Lexer {
            stream: Stream::new(&String),
            checkpoint: Position::new(0, 0),
        }
    }


    fn set_checkpoint(&mut self) {
        self.checkpoint = stream.pos();
    }
}

impl Iterator for Lexer<'a> {
    type Output = Result<Token>;
    fn next(&mut self) -> Option<Result<Token>> {
        self.set_checkpoint();
        let option = stream.peek();

        if option == None {
            return None
        }

        else if cs::DIGIT_INIT.contains(char) {
            self.read_digit()
        }

        else if cs::SYMBOL_INIT.contains(char) {
            self.read_symbol()
        }

        else if cs::SPECIAL_INIT.contains(char) { 
            // punctuation
            self.read_digit()
        }

        else if cs::STRING_INIT.contains(char) {
            self.read_string()
        }
        
        else if cs::CHAR_INIT.contains(char) {
            // 'a'
            self.read_char()
        }

        else if cs::C.contains(char) {
            
            self.read_digit()
        }

        else {Some(
            Err("Disallowed character")
        )}


    }

    // ready
    fn read_digit(&mut self) {
        let string = self.read_integer();
        let char = self.stream.peek()
        if char == '.' {
            let string.push_str(self.read_integer());

            if self.stream.peek() != 'i' {
                return Token::Float(string);
            }
            else {
                self.stream.next();
                string.push('i')
                return Token::Complex(string);
            } 
        }
        else if char == 'i' {
            self.stream.next();
            string.push('i')
            return Token::Float(string)
        }
        else {
            return Token::Integer(string)
        }
    }
    // ready
    fn read_integer(&mut self) {
        let out = String::from(self.stream.peek());
        loop {
            let option = self.stream.next();

            if option != None {
                let char = option.unwrap();
                if cs::INT.contains(char){
                    out.push(char);
                    continue
                }
                else {
                    return out
                }   
            }
            else {
                return out
            }
        }
    }



}

    
    fn read_id(&mut self) {

        out 
        loop {

            let option = self.stream.next();
            
        }

    }

    fn read_word(&mut self) {
    }


}
