


type Position = (usize, usize);

struct ParsingError {
    syntax_error: SyntaxError
    file_path: String,
    hint: Option<String>
}


impl ParsingError {


    fn start(&self) -> usize {
        self.syntax_error.start
    }
}