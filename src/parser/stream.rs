// use crate::errors::Position;

#[derive(Clone, Copy)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Position {
        return Position { row, col };
    }
}

pub struct Stream<'a> {
    iterator: std::str::Chars<'a>,
    char: char,
    eof: bool,
    position: Position,
}

impl<'a> From<&'a String> for Stream<'a> {
    fn from(string: &'a String) -> Stream<'a> {
        let iterator = string.chars();
        let char = '\n';
        let eof = false;
        let position = Position { row: 0, col: 0 };
        Stream {
            iterator,
            char,
            eof,
            position,
        }
    }
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(string: &'a str) -> Stream<'a> {
        let iterator = string.chars();
        let char = '\n';
        let eof = false;
        let position = Position { row: 0, col: 0 };
        Stream {
            iterator,
            char,
            eof,
            position,
        }
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
    pub fn walk_while(&mut self, char_set: &String) -> String {
        let mut out = String::from(self.char);
        for c in self {
            if ! char_set.contains(c) {
                break;
            }
            out.push(c);
        }
        out
    }

    pub fn walk_while_not(&mut self, char_set: &String) -> String {
        let mut out = String::from(self.char);
        for c in self {
            if char_set.contains(c) {
                break;
            }
            out.push(c);
        }
        out
    }
}
