use crate::settings::{FOLLOWING_LINES, PREVIOUS_LINES};
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub number: i32,
    pub content: String,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Position {
        return Position { row, col };
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    pub fn new(string: &str, row: i32, cols: (i32, i32)) -> SyntaxError {
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
    fn get_lines(string: &str, row: i32) -> (Line, Vec<Line>, Vec<Line>) {
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

    pub fn simple_display(&self) -> String {
        let len = format!("{}", self.line.number + 1).len();
        let mut space = String::new();
        for _ in 0..len {
            space.push(' ');
        }
        format!(
            "{} |\n{} | {}\n{} | {}\nSyntax Error: {}",
            space,
            self.line.number + 1,
            self.line.content,
            space,
            self.underline,
            self.message
        )
    }
}
