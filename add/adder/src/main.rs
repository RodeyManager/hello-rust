use add_one;
use add_two;

fn main() {
    let a = 1;
    println!("Hello, world!");
    println!("a add one equal: {}; ", add_one::add_one(a));
    println!("a add two equal: {}", add_two::add_two(a));
}
