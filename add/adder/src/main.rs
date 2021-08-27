use add_one;
use add_two;

fn main() {
    let n = 10;
    println!("Hello, add-one={}", add_one::add_one(n));
    println!("Hello, add_two={}", add_two::add2(n));
}
