// #![allow(dead_code)]
mod stream;

pub struct ErrorHighlight {
    pub line           : Line,
    pub message        : String,
    pub underline      : String,
    pub previous_lines : [Line; 4],
    pub following_lines: [Line; 4],
}


pub struct Line {
    pub number: i32,
    pub content: String
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: i32,
    pub col: i32
}

pub struct Stream<'a> {
    iterator: std::str::Chars<'a>,
    pub char: char,
    eof     : bool,
    position: Position,
}

impl<'a> Stream<'a> {

    pub fn from(string: &'a String) -> Stream<'a> {
        let mut iterator = string.chars();
        let char = iterator.next().unwrap();
        let eof  = false;
        let position = Position {row: 0, col: 0};
        Stream { iterator,
                 char,
                 eof,
                 position
             }
    }

    pub fn next(&mut self) -> Option<char> {
        let option = self.iterator.next();
        match option {
            None => {
                self.eof = true;
                option
            },
            Some(char) => {
                self.char = char;
                if self.char == '\n' {
                    self.position.col  = 0;
                    self.position.row += 1;
                }
                else {
                    self.position.col += 1;
                }
                option
            }
        }
    }

    pub fn walk_while(&mut self, char_set: &String) -> String {
        let mut out = String::from(self.char);
        loop {
            let option = self.next();
            if option == None {
                break;
            }
            let char = option.unwrap();

            if !char_set.contains(char) {
                break;
            }
            out.push(char);
        }
        return out;
    }

    pub fn walk_while_not(&mut self, char_set: &String) -> String {
        let mut out = String::from(self.char);
        loop {
            let option = self.next();
            if option == None {
                break;
            }
            let char = option.unwrap();

            if char_set.contains(char) {
                break;
            }
            out.push(char);
        }
        return out;
    }

    pub fn eof(&self) -> bool {
        self.eof
    }

    pub fn pos(&self) -> Position {
        self.position
    }
}


fn main() {
    let string = "1341 23sda s3dasd".to_string();


    for substring in string.split(" ") {
        println!("{:?}", substring);
    }


}
