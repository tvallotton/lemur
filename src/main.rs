#![allow(dead_code)]
#![allow(unused_imports)]

mod parser;
mod settings; 

struct Myiterator {
    vector: Vec<i32>,
    index: i32,
}

impl Iterator for Myiterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        self.index += 1;
        if self.vector.len() <= self.index as usize {
            None
        }
        else {
            Some(self.vector[self.index as usize])
        }
    }
}


fn main() {
   
    

    
    println!("{}", "sad".contains("x"));

    

}
