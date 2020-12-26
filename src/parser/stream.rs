use super::errors::Position;



pub struct Stream<'a> {
    string: &'a str,
    iterator: std::str::Chars<'a>,
    char: char,
    eof: bool,
    position: Position,
    start_index: usize,
    end_index: usize,
}

impl<'a> From<&'a String> for Stream<'a> {
    fn from(string: &'a String) -> Stream<'a> {
        let iterator = string.chars();
        let char = '\0';
        let eof = false;
        let position = Position { row: 0, col: 0 };
        let start_index = 0;
        let end_index = 0;
        let mut stream = Stream {
            string,
            iterator,
            char,
            eof,
            position,
            start_index,
            end_index,
        };
        stream.next();
        stream
    }
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(string: &'a str) -> Stream<'a> {
        let iterator = string.chars();
        let char = '\0';
        let eof = false;
        let position = Position { row: 0, col: 0 };
        let start_index = 0;
        let end_index = 0;
        let mut stream = Stream {
            string,
            iterator,
            char,
            eof,
            position,
            start_index,
            end_index,
        };
        stream.next();
        stream
    }
}

impl<'a> Iterator for Stream<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let option = self.iterator.next();
        match option {
            None => {
                self.eof = true;
                option
            }
            Some(char) => {
                self.end_index += char.len_utf8();
                self.char = char;
                if self.char == '\n' {
                    self.position.col = 0;
                    self.position.row += 1;
                } else {
                    self.position.col += 1;
                }
                option
            }
        }
    }
}

impl<'a> Stream<'a> {
    pub fn eof(&self) -> bool {
        self.eof
    }

    pub fn pos(&self) -> Position {
        self.position
    }
    pub fn peek(&self) -> char {
        self.char
    }
    pub fn walk_while(&mut self, char_set: &str) -> &'a str {
        self.begin_slice();

        while let Some(c) = self.next() {
            if !char_set.contains(c) {
                break;
            }
        }
        self.get_slice()
    }

    pub fn begin_slice(&mut self) {
        self.start_index = self.end_index - self.char.len_utf8();
    }

    pub fn get_slice(&self) -> &'a str {
        &self.string[self.start_index..(self.end_index - self.char.len_utf8())]
    }

    pub fn walk_while_not(&mut self, char_set: &str) -> &'a str {
        self.begin_slice();
        while let Some(c) = self.next() {
            if char_set.contains(c) {
                break;
            }
        }
        self.get_slice()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iterator() {
        let string = String::from("sadknf asdjfek \nasdfm");
        let stream = Stream::from(&string);

        for (c0, c1) in stream.zip(string.chars()) {
            assert_eq!(c0, c1);
        }
    }
}
