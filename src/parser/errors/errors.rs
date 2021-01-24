use crate::settings::{FOLLOWING_LINES, PREVIOUS_LINES};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        return Position { row, col };
    }
}

type Location = (Position, Position);

#[derive(Clone, PartialEq, Debug)]
pub struct SyntaxError {
    pub start: Position,
    pub end: Position,
    pub message: String,
}

impl SyntaxError {
    pub fn new(message: String, start: Position, end: Position) -> SyntaxError {
        SyntaxError {
            message,
            start,
            end,
        }
    }

    fn multiline(self) -> bool {
        self.start.row != self.end.row
    }

    pub fn simple_display(&self, module: &str) -> String {
        for (i, line) in module.split("\n").enumerate() {
            if i == self.start.row {
                return format_line(self, line);
            }
        }
        String::new()
    }
}

fn format_line(error: &SyntaxError, line: &str) -> String {
    let spaces = get_spaces(error.start.row);

    let underline = get_underline(error);
    format!("{spaces}|\n {} | {}\n{spaces}|{}\nSyntaxError: {}", 
            error.start.row+1, 
            line,
            underline, 
            error.message,
            spaces = spaces, )
}

fn get_underline(error: &SyntaxError) -> String {
    let mut out = String::new();
    for _ in 0..error.start.col {
        out.push(' ');
    }
    for _ in error.start.col..error.end.col {
        out.push('^');
    }
    out
}
fn get_spaces(number: usize) -> String {
    let mut out = String::from("  ");
    for _ in format!("{}", number).chars() {
        out.push(' ');
    }
    out
}

