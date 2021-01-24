


type Position = (usize, usize);

struct ParsingError {
    pub message: String,
    pub start: Position,
    pub end: Position,
}