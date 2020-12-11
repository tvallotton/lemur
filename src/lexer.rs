





use stream::Stream;
use stream::Position;







struct Lexer<'a> {
    stream: Stream<'a>,
    check_point: Position,
}

impl Lexer<'a> {


    fn next(&mut self) -> Option<Result<Token>> {


        let option = stream.next();

        if option == None {
            return None
        }

        if cs::DIGIT_INIT.contains(char) {
            return self.read_digit()
        }


        else {
            return Err("Disallowed character")
        }


    }

    fn read_digit(&mut self) {


        result = self.read_integer();

        match result {
            Err(string) => self.read_float(string)
            _ => result
        }
    }
    fn read_integer(&mut self) {

        let out = String;

        loop {

            let option = self.stream.next();

            match option {

                None => Ok(out),
                Some('\n') => Ok(out),
                Some(' ') => Ok(out),
                Some(char) => {
                    if cs::INT.contains(char) {
                        out
                    }
                }
            }
        }

    }

    }
    fn read_word(&mut self) {
    }

    fn read_word(&mut self) {
    }


}
