#![allow(dead_code)]
#![allow(unused_imports)]

mod errors;
mod settings;
mod stream;



fn main() {
    
    let iterator = 1..100;


    for i in iterator {
        println!("{}", i);
    }


}
