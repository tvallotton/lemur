

use crate::settings::{
    PREVIOUS_LINES,
    FOLLOWING_LINES
};

pub struct Line {
    pub number: i32,
    pub content: String
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: i32,
    pub col: i32
}

impl Position {
    fn new(row: i32, col:i32) -> Position {
        return Position {
            row,
            col
        }
    }
}

pub struct SyntaxError {
    pub line           : Line,
    pub message        : String,
    pub underline      : String,
    pub previous_lines : Vec<Line>,
    pub following_lines: Vec<Line>,
}





// impl SyntaxError {

//     pub fn new(string: &String, row: i32, cols: [i32; 2]) -> SyntaxError {

//         let lines = SyntaxError::get_lines(string, row, cols);

//         let target_line = lines;

//         return ;
//     }


//     fn get_lines(string: &String, row: i32, cols: [i32; 2]) -> (Line, Line, Line) {
//         let previous_lines  = Vec::new();
//         let following_lines = Vec::new();
//         let target_line;
//         for (num, line) in string.split("\n").enumerate() {
//             if row - PREVIOUS_LINES  < num < row {
//                 previous_lines.push(Line {
//                     number: num,
//                     content: line});
//             }
//             else if row < num < row + FOLLOWING_LINES {
//                 following_lines.push(Line {
//                     number: num,
//                     content: line
//                 })
//             }

//             else if row == num {
//                 target_line = Line {
//                     number: num,
//                     content: line
//                 }
//             }
//             else if num > row {
//                 break
//             }
//         }
//         return (previous_lines, following_lines,target_line)
//     }

//     fn underline(target_line: Line, row: i32, cols: [i32; 2]) {
//         let underline = String::new();
//         for i in target_line.contant.length() {

//             if i < cols[0] {
//                 underline.push(' ');
//                 continue
//             }
//             else if cols[0] < i && i < cols[1] {
//                 underline.push('^');
//             }
//             else {
//                 return underline;
//             }
//         }
//     }
// }


// pub fn underline(target_line: Line, row: i32, cols: [i32; 2]) {
//     let underline = String::new();
//     for i in target_line.contant.length() {

//         if i < cols[0] {
//             underline.push(' ');
//             continue
//         }
//         else if cols[0] < i && i < cols[1] {
//             underline.push('^');
//         }
//         else {
//             return underline;
//         }
//     }
// }
