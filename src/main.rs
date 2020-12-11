

use std::io::stdin;

fn main() {
    println!("Say your name bitch: ");
    let mut line = String::new();

    let b1 = stdin().read_line(&mut line);
    println!("Hello, {}", line);
    println!("this is b1: {:?}", b1);
}
