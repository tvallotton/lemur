const PREVIOUS_LINES: i32 = 1;
const FOLLOWING_LINES: i32 = 1;
// use crate::settings::{
//     PREVIOUS_LINES,
//     FOLLOWING_LINES
// };

pub struct Line {
    pub number: i32,
    pub content: String,
}

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

pub struct SyntaxError {
    pub line: Line,
    pub message: String,
    pub underline: String,
    pub previous_lines: Vec<Line>,
    pub following_lines: Vec<Line>,
}

impl Line {
    fn init() -> Line {
        return Line {
            content: String::new(),
            number: 0,
        };
    }
}

impl SyntaxError {
    // Listo y testedo
    pub fn new(string: &String, row: i32, cols: (i32, i32)) -> SyntaxError {
        let (target_line, previous_lines, following_lines) = SyntaxError::get_lines(string, row);
        let message = String::new();
        let underline = SyntaxError::get_underline(cols, '^');

        return SyntaxError {
            line: target_line,
            previous_lines,
            following_lines,
            message,
            underline,
        };
    }
    // Listo y testedo
    fn get_lines(string: &String, row: i32) -> (Line, Vec<Line>, Vec<Line>) {
        let mut previous_lines = Vec::new();
        let mut following_lines = Vec::new();
        let mut target_line = Line::init();
        for (number, line) in string.split("\n").enumerate() {
            let num = number as i32;
            if row - PREVIOUS_LINES < num && num < row {
                previous_lines.push(Line {
                    number: num,
                    content: String::from(line),
                });
            } else if row == num {
                target_line = Line {
                    number: num,
                    content: String::from(line),
                };
            } else if row < num && num < row + FOLLOWING_LINES {
                following_lines.push(Line {
                    number: num,
                    content: String::from(line),
                });
            } else if num > row + FOLLOWING_LINES {
                break;
            }
        }
        return (target_line, previous_lines, following_lines);
    }
    // Listo y testedo
    fn get_underline(cols: (i32, i32), char: char) -> String {
        let mut out = String::new();
        for _ in 0..(cols.0) {
            out.push(' ');
        }
        for _ in cols.0..cols.1 {
            out.push(char);
        }
        return out;
    }
}

// Tests
fn main() {
    let string = String::from("asdasjkd sdnjdf s\ndjf n- 32m asdmjasm,d on\njknm2324r n\nfkalñs3mr '0hj 45jgir\nof qwefrpig20 i\n3ef0grosavsf d a32 j");

    println!("{}", string);

    let se = SyntaxError::new(&string, 3, (2, 7));

    println!("{}\n{}\n", se.line.content, se.underline);
}
