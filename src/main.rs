#![allow(dead_code)]
#![allow(unused_imports)]

// mod parser;
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
    let mut array = Myiterator {
        vector: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        index: 0,
    };

    for element in &mut array {
        println!("{}", element);
        if element == 6 {
            break;
        }
    }
    println!("break");

    println!("{:?}", array.next());
    println!("still on break");
    for element in &mut array {
        println!("{}", element);
    }

}
